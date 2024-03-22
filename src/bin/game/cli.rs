#![warn(clippy::all)]
// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]
use std::io::Write;

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

    let wl = BuiltinWList::default();
    let builder = game::Game::builder()
        .length(cli.length)
        .max_steps(cli.max_steps)
        .precompute(cli.precompute)
        .wordlist(wl);
    let mut game = builder.build()?;

    debug!("{game:#?}");

    let mut response: GuessResponse;
    let mut guess: Word;
    loop {
        guess = get_word(&cli, game.step())?;
        response = match game.guess(guess) {
            Ok(r) => r,
            Err(err) => match err {
                GameError::GuessHasWrongLength => {
                    println!("word length: must be {} long", game.length());
                    continue;
                }
                _ => {
                    return Err(err.into());
                }
            },
        };

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

fn get_word(_cli: &Cli, step: usize) -> std::io::Result<Word> {
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
