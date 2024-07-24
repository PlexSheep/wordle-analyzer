use wordle_analyzer::game::Game;
use wordle_analyzer::solve::{AnyBuiltinSolver, NaiveSolver, Solver, StupidSolver};
use wordle_analyzer::wlist::builtin::BuiltinWList;
use wordle_analyzer::wlist::word::{Word, WordData};
use wordle_analyzer::wlist::WordList;

fn wordlist() -> impl WordList {
    BuiltinWList::default()
}

#[test]
fn test_build_builtin_solvers() {
    let wl = wordlist();
    let stupid_solver =
        AnyBuiltinSolver::Stupid(StupidSolver::build(&wl).expect("could not build naive solver"));
    let naive_solver =
        AnyBuiltinSolver::Naive(NaiveSolver::build(&wl).expect("could not build naive solver"));
}

#[test]
fn test_naive_play_predetermined_game() -> anyhow::Result<()> {
    let wl = wordlist();
    let sl =
        AnyBuiltinSolver::Naive(NaiveSolver::build(&wl).expect("could not build naive solver"));
    let mut game = Game::build(5, false, 6, &wl, false)?;
    game.set_solution(Some(("nines".into(), 0.002))); // The accuracy is made up but shouldn't
                                                      // matter
    sl.make_a_move(&mut game)?;
    assert_eq!(
        game.responses().last().unwrap().guess(),
        &Word::from("which")
    );
    sl.make_a_move(&mut game)?;
    assert_eq!(
        game.responses().last().unwrap().guess(),
        &Word::from("their")
    );
    sl.make_a_move(&mut game)?;
    assert_eq!(
        game.responses().last().unwrap().guess(),
        &Word::from("being")
    );
    sl.make_a_move(&mut game)?;
    assert_eq!(
        game.responses().last().unwrap().guess(),
        &Word::from("since")
    );
    sl.make_a_move(&mut game)?;
    assert_eq!(
        game.responses().last().unwrap().guess(),
        &Word::from("lines")
    );
    sl.make_a_move(&mut game)?;
    assert_eq!(
        game.responses().last().unwrap().guess(),
        &Word::from("mines")
    );
    sl.make_a_move(&mut game)?;
    assert_eq!(
        game.responses().last().unwrap().guess(),
        &Word::from("wines")
    );

    // naive is at the moment too bad to solve "nines"
    assert!(game.finished());
    assert!(!game.won());

    Ok(())
}

#[test]
fn test_naive_play_predetermined_game_by_manual_guess_and_evak() -> anyhow::Result<()> {
    let wl = wordlist();
    let sl =
        AnyBuiltinSolver::Naive(NaiveSolver::build(&wl).expect("could not build naive solver"));
    // we don't insert the solution yet,
    // pretend that a user inputs guesses manually
    let mut game = Game::build(5, false, 6, &wl, false)?;
    let _actual_solution: Option<WordData> = Some(("nines".into(), 0.002));
    let mut next_guess;

    next_guess = sl.guess_for(&game)?;
    assert_eq!(next_guess, Word::from("which"));
    game.guess(next_guess, Some("xxfxx".into()))?;

    next_guess = sl.guess_for(&game)?;
    assert_eq!(next_guess, Word::from("their"));
    game.guess(next_guess, Some("xxffx".into()))?;

    next_guess = sl.guess_for(&game)?;
    assert_eq!(next_guess, Word::from("being"));
    game.guess(next_guess, Some("xfffx".into()))?;

    next_guess = sl.guess_for(&game)?;
    assert_eq!(next_guess, Word::from("since"));
    game.guess(next_guess, Some("fcfxf".into()))?;

    next_guess = sl.guess_for(&game)?;
    assert_eq!(next_guess, Word::from("lines"));
    game.guess(next_guess, Some("xcccc".into()))?;

    next_guess = sl.guess_for(&game)?;
    assert_eq!(next_guess, Word::from("mines"));
    game.guess(next_guess, Some("xcccc".into()))?;

    next_guess = sl.guess_for(&game)?;
    assert_eq!(next_guess, Word::from("wines"));
    game.guess(next_guess, Some("xcccc".into()))?;

    // naive is at the moment too bad to solve "nines"
    assert!(game.finished());
    assert!(!game.won());

    Ok(())
}
