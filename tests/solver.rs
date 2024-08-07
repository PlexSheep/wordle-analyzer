use test_log::test; // set the log level with an envvar: `RUST_LOG=trace cargo test`

use wordle_analyzer::solve::{AnyBuiltinSolver, NaiveSolver, Solver, StupidSolver};
use wordle_analyzer::wlist::builtin::BuiltinWList;
use wordle_analyzer::wlist::WordList;

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
