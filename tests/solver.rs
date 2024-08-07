use test_log::test; // set the log level with an envvar: `RUST_LOG=trace cargo test`

use wordle_analyzer::game::evaluation::Evaluation;
use wordle_analyzer::game::Game;
use wordle_analyzer::solve::{AnyBuiltinSolver, NaiveSolver, Solver, StupidSolver};
use wordle_analyzer::wlist::builtin::BuiltinWList;
use wordle_analyzer::wlist::word::{Word, WordData};
use wordle_analyzer::wlist::WordList;

use rayon::prelude::*;

fn wordlist() -> impl WordList {
    BuiltinWList::default()
}

#[test]
fn test_build_builtin_solvers() {
    let wl = wordlist();
    let _stupid_solver =
        AnyBuiltinSolver::Stupid(StupidSolver::build(&wl).expect("could not build naive solver"));
    let _naive_solver =
        AnyBuiltinSolver::Naive(NaiveSolver::build(&wl).expect("could not build naive solver"));
}

#[test]
fn test_naive_win_games() -> anyhow::Result<()> {
    let wl = wordlist();
    let sl =
        AnyBuiltinSolver::Naive(NaiveSolver::build(&wl).expect("could not build naive solver"));
    let builder = Game::builder(&wl);

    { 0..50 }.into_par_iter().for_each(|_round| {
        let mut game = builder.build().expect("could not make game");
        sl.play(&mut game).expect("could not play game");
        assert!(game.finished());
        assert!(game.won());
    });
    Ok(())
}
