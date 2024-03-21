#![warn(clippy::all)]
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]
use std::io::Write;

use anyhow::anyhow;
use clap::Parser;
use libpt::log::*;
use wordle_analyzer::error::GameError;
use wordle_analyzer::game::response::GuessResponse;

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
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    if cli.verbose {
        Logger::build_mini(Some(Level::TRACE))?;
    } else {
        Logger::build_mini(Some(Level::INFO))?;
    }
    debug!("dumping CLI: {:#?}", cli);

    let mut game = game::Game::<BuiltinWList>::builder()
        .length(cli.length)
        .precompute(cli.precompute)
        .build()?;

    debug!("{game:#?}");

    let mut response: GuessResponse;
    let mut guess: Word;
    loop {
        guess = match get_word(&cli, game.step()) {
            Ok(g) => g,
            Err(err) => match err.downcast::<GameError>() {
                Ok(game_err) => match game_err {
                    GameError::GuessHasWrongLength => {
                        println!("wring length: must be {} long", game.length());
                        continue;
                    }
                    _ => {
                        return Err(game_err.into());
                    }
                },
                Err(err) => return Err(anyhow!(err.to_string())),
            },
        };
        response = game.guess(guess)?;

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

fn get_word(_cli: &Cli, step: usize) -> anyhow::Result<Word> {
    let mut word = Word::new();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // TODO: get user input
    // TODO: validate user input

    print!("guess {step} > ");
    stdout.flush()?;
    stdin.read_line(&mut word)?;
    word = word.replace('\n', "");

    Ok(word)
}
