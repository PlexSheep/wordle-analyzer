#![warn(clippy::all)]
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]

use clap::Parser;
use libpt::log::*;

use wordle_analyzer::game::response::GuessResponse;

use wordle_analyzer::solve::{stupid, BuiltinSolvers, Solver};
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
    #[arg(short, long)]
    verbose: bool,
    /// which solver to use
    #[arg(short, long, default_value_t = BuiltinSolvers::default())]
    solver: BuiltinSolvers,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    if cli.verbose {
        Logger::build_mini(Some(Level::TRACE))?;
    } else {
        Logger::build_mini(Some(Level::INFO))?;
    }
    debug!("dumping CLI: {:#?}", cli);

    let wl = BuiltinWList::default();
    let builder = game::Game::builder(&wl)
        .length(cli.length)
        .max_steps(cli.max_steps)
        .precompute(cli.precompute);
    let solver = match cli.solver {
        BuiltinSolvers::Naive => stupid::StupidSolver::build(&wl)?,
        _ => todo!(),
    };
    let mut game = builder.build()?;

    debug!("{game:#?}");

    let mut response: GuessResponse;
    let mut _guess: Word;
    loop {
        response = solver.play(&mut game)?;
        println!("{response}");

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
