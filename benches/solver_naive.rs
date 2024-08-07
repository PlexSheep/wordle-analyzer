use wordle_analyzer::bench::builtin::BuiltinBenchmark;
use wordle_analyzer::bench::Benchmark;
use wordle_analyzer::game::{self, GameBuilder};
use wordle_analyzer::solve::{NaiveSolver, Solver};
use wordle_analyzer::wlist::builtin::BuiltinWList;

fn main() -> anyhow::Result<()> {
    let wl = BuiltinWList::english(5);
    let builder: GameBuilder<'_, BuiltinWList> = game::Game::builder(&wl)
        .length(5)
        .max_steps(6)
        .precompute(true);
    let solver: NaiveSolver<_> = NaiveSolver::build(&wl)?;
    let bench = BuiltinBenchmark::build(&wl, solver, builder, 16)?;
    bench.start(2000, &bench.builder())?;
    println!("{}", bench.report());
    Ok(())
}
