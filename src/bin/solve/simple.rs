#![warn(clippy::all)]
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]

use clap::{Parser, Subcommand};
use libpt::cli::console::style;
use libpt::cli::{repl::Repl, strum};
use libpt::log::*;
use strum::EnumIter;

use wordle_analyzer::error::Error;
use wordle_analyzer::game::evaluation::Evaluation;
use wordle_analyzer::game::response::GuessResponse;

use wordle_analyzer::solve::{BuiltinSolverNames, Solver};
use wordle_analyzer::wlist::builtin::BuiltinWList;
use wordle_analyzer::wlist::word::{Word, WordData};
use wordle_analyzer::wlist::WordList;
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
    #[arg(long, default_value_t = BuiltinSolverNames::default())]
    solver: BuiltinSolverNames,

    /// set if the solver should play a full native game without interaction
    #[arg(short, long)]
    non_interactive: bool,

    // FIXME: line breaks don't work correctly in the cli help
    //
    /// Solution for the game
    ///
    /// This will only be used when non-interactive is used. You can use this option to see how the
    /// selected solver behaves when trying to guess a specific solution, which can help reproduce
    /// behavior.
    #[arg(short, long)]
    solution: Option<Word>,
}

#[derive(Subcommand, Debug, EnumIter, Clone)]
enum ReplCommand {
    /// Let the user input the response to the last guess
    ///
    Response { encoded: String },
    /// Let the user input a word and the response for that word
    ///
    /// Evaluation Format:
    ///
    /// 'x' means wrong character
    ///
    /// 'p' means present character
    ///
    /// 'c' means correct character
    ///
    /// Example:
    ///
    /// 'xxxcc' means the first 3 chars are wrong but the second 2 chars are correct
    ///
    /// 'xppxc' means the first character is wrong, the next two characters are present, the last
    /// is correct
    Guess {
        your_guess: String,
        evalutation: String,
    },
    /// Let the solver make a guess
    Solve,
    /// Show the current state of the game
    Show,
    /// Display data about the wordlist
    Wl {
        #[command(subcommand)]
        cmd: WlCommand,
    },
    /// Leave the Repl
    Exit,
}

#[derive(Subcommand, Debug, EnumIter, Clone, Default)]
enum WlCommand {
    #[default]
    Stats,
    Top {
        amount: usize,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    Logger::builder()
        .set_level(cli.verbose.level())
        .build()
        .unwrap();
    trace!("dumping CLI: {:#?}", cli);

    if cli.non_interactive {
        play_native_non_interactive(cli)?;
        std::process::exit(0);
    }
    help_guess_interactive(cli)
}

fn help_guess_interactive(cli: Cli) -> anyhow::Result<()> {
    let wl = BuiltinWList::default();
    let builder = game::GameBuilder::new(&wl, false)
        .length(cli.length)
        .max_steps(cli.max_steps)
        .precompute(cli.precompute);
    let solver = cli.solver.to_solver(&wl);
    let mut game = builder.build()?;

    let mut repl = libpt::cli::repl::DefaultRepl::<ReplCommand>::default();

    debug!("entering the repl");
    loop {
        // repl.step() should be at the start of your loop
        // It is here that the repl will get the user input, validate it, and so on
        match repl.step() {
            Ok(c) => c,
            Err(e) => {
                // if the user requested the help, print in blue, otherwise in red as it's just an
                // error
                if let libpt::cli::repl::error::Error::Parsing(e) = &e {
                    if e.kind() == clap::error::ErrorKind::DisplayHelp {
                        println!("{}", style(e).cyan());
                        continue;
                    }
                }
                println!("{}", style(e).red().bold());
                continue;
            }
        };

        // now we can match our defined commands
        //
        // only None if the repl has not stepped yet
        match repl.command().to_owned().unwrap() {
            ReplCommand::Exit => break,
            ReplCommand::Wl { cmd } => wlcommand_handler(&cli, &cmd, &wl)?,
            ReplCommand::Show => {
                println!("{}", game);
            }
            ReplCommand::Solve => {
                let best_guess = solver.guess_for(&game)?;
                debug!("game state: {game:?}");
                println!("best guess: {best_guess}");
            }
            ReplCommand::Guess {
                your_guess,
                evalutation,
            } => {
                let evaluation_converted: Evaluation =
                    Evaluation::build(&your_guess, &evalutation)?;
                let guess = game.guess(your_guess, Some(evaluation_converted));
                debug!("your guess: {guess:?}");
                if guess.is_err() {
                    eprintln!("{}", style(guess.unwrap_err()).red().bold());
                    continue;
                }
                println!("{}", guess.unwrap());
                debug!("game state: {game:#?}");
            }
            _ => todo!(),
        }
    }
    Ok(())
}

fn wlcommand_handler(_cli: &Cli, cmd: &WlCommand, wl: &impl WordList) -> anyhow::Result<()> {
    match cmd {
        WlCommand::Stats => {
            println!("{wl}")
        }
        WlCommand::Top { amount } => {
            println!();
            for s in wl.n_most_likely(*amount).iter() {
                println!("\t\"{}\":\t{:.08}%", s.0, s.1 * 100.0);
            }
        }
    }
    Ok(())
}

fn play_native_non_interactive(cli: Cli) -> anyhow::Result<()> {
    let wl = BuiltinWList::default();
    let mut builder = game::Game::builder(&wl)
        .length(cli.length)
        .max_steps(cli.max_steps)
        .precompute(cli.precompute);
    if cli.solution.is_some() {
        let solw: Word = cli.solution.unwrap();
        let sol = wl.get_word(&solw);
        if sol.is_none() {
            eprintln!("the requested solution \"{solw}\" is not in the wordlist");
            return Err(Error::GameError {
                source: wordle_analyzer::error::GameError::WordNotInWordlist(solw),
            }
            .into());
        }
        builder = builder.solution(sol);
    }
    let solver = cli.solver.to_solver(&wl);
    let mut game = builder.build()?;

    debug!("{game:#?}");

    let mut response: GuessResponse;
    let mut _guess: Word;
    loop {
        response = solver.make_a_move(&mut game)?;
        debug!("game state: {game:#?}");
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
