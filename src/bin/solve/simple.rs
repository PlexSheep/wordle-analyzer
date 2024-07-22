#![warn(clippy::all)]
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]

use clap::Parser;
use libpt::cli::{repl::Repl, strum};
use libpt::log::*;
use strum::IntoEnumIterator;

use wordle_analyzer::game::response::GuessResponse;

use wordle_analyzer::solve::{BuiltinSolverNames, Solver};
use wordle_analyzer::wlist::builtin::BuiltinWList;
use wordle_analyzer::wlist::word::Word;
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
    #[command(flatten)]
    verbose: libpt::cli::args::VerbosityLevel,
    /// which solver to use
    #[arg(short, long, default_value_t = BuiltinSolverNames::default())]
    solver: BuiltinSolverNames,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    Logger::builder()
        .set_level(cli.verbose.level())
        .build()
        .unwrap();
    trace!("dumping CLI: {:#?}", cli);

    // let repl = libpt::cli::repl::DefaultRepl::<ReplCommand>::default();

    let wl = BuiltinWList::default();
    let builder = game::Game::builder(&wl)
        .length(cli.length)
        .max_steps(cli.max_steps)
        .precompute(cli.precompute);
    let solver = cli.solver.to_solver(&wl);
    let mut game = builder.build()?;

    debug!("{game:#?}");

    let mut response: GuessResponse;
    let mut _guess: Word;
    loop {
        response = solver.make_a_move(&mut game)?;
        println!("{}. guess: {response}", game.step() - 1);

        if response.finished() {
            break;
        }
    }
    if response.won() {
        println!("You win! You took {} guesses.", game.step() - 1);
    } else {
        println!("You lose! The solution was {:?}.", game.solution());
    }

    Ok(())
}
