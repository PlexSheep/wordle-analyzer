#![warn(clippy::all)]
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]

use std::sync::Arc;

use clap::Parser;
use libpt::log::*;

use wordle_analyzer::bench::builtin::BuiltinBenchmark;
use wordle_analyzer::bench::report::Report;
use wordle_analyzer::bench::{Benchmark, DEFAULT_N};
use wordle_analyzer::error::WResult;
use wordle_analyzer::solve::{BuiltinSolverNames, Solver};
use wordle_analyzer::wlist::builtin::BuiltinWList;

use wordle_analyzer::{self, game};

#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about, author)]
struct Cli {
    /// precompute all possibilities for better performance at runtime
    #[arg(short, long)]
    precompute: bool,
    /// how long should the word be?
    #[arg(short, long, default_value_t = wordle_analyzer::DEFAULT_WORD_LENGTH)]
    length: usize,
    /// how many times can we guess?
    #[arg(short, long, default_value_t = wordle_analyzer::DEFAULT_MAX_STEPS)]
    max_steps: usize,
    /// more verbose logs
    #[arg(short, long)]
    verbose: bool,
    /// which solver to use
    #[arg(short, long, default_value_t = BuiltinSolverNames::default())]
    solver: BuiltinSolverNames,
    /// how many games to play for the benchmark
    #[arg(short, long, default_value_t = DEFAULT_N)]
    n: usize,
    /// how many threads to use for benchmarking
    ///
    /// Note that the application as the whole will use at least one more thread.
    #[arg(short, long, default_value_t = num_cpus::get())]
    threads: usize,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    if cli.verbose {
        Logger::build_mini(Some(Level::DEBUG))?;
    } else {
        Logger::build_mini(Some(Level::INFO))?;
    }
    trace!("dumping CLI: {:#?}", cli);

    let wl = BuiltinWList::default();
    let builder = game::Game::builder(&wl)
        .length(cli.length)
        .max_steps(cli.max_steps)
        .precompute(cli.precompute);
    let solver = cli.solver.to_solver(&wl);
    let bench = Arc::new(BuiltinBenchmark::build(&wl, solver, builder, cli.threads)?);
    let bench_running = bench.clone();
    trace!("{bench:#?}");
    let n = cli.n;
    let bench_th: std::thread::JoinHandle<WResult<Report>> =
        std::thread::spawn(move || bench_running.bench(n));

    while !bench_th.is_finished() {
        println!("{}", bench.report());
    }

    // finished report
    println!("{}", bench_th.join().expect("thread go boom")?);

    Ok(())
}
