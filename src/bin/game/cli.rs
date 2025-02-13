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
    /// select a wordlist
    ///
    /// 'ger' and 'eng' are special values bundled with this executable, if the value does not
    /// match either of those, it will be assumed to be a file path.
    #[arg(short, long, default_value_t = String::from("eng"))]
    wordlist: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    if cli.verbose {
        Logger::builder().set_level(Level::DEBUG).build().unwrap();
    } else {
        Logger::builder().set_level(Level::INFO).build().unwrap();
    }
    debug!("dumping CLI: {:#?}", cli);

    let wl = match cli.wordlist.as_str() {
        "ger" => BuiltinWList::german(cli.length),
        "eng" => BuiltinWList::english(cli.length),
        _ => BuiltinWList::load(&cli.wordlist, cli.length)?,
    };
    let builder = game::Game::builder(&wl)
        .length(cli.length)
        .max_steps(cli.max_steps)
        .precompute(cli.precompute);
    let mut game = builder.build()?;

    debug!("{game:#?}");

    let mut response: GuessResponse;
    let mut guess: Word;
    loop {
        guess = get_word(&cli, game.step())?;
        response = match game.guess(&guess, None) {
            Ok(r) => r,
            Err(err) => match err {
                GameError::GuessHasWrongLength(len) => {
                    println!("word length: must be {} long but is {}", game.length(), len);
                    continue;
                }
                GameError::WordNotInWordlist(w) => {
                    println!("bad word: word \"{w}\" is not in the wordlist",);
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

    print!("guess {step} > ");
    stdout.flush()?;
    stdin.read_line(&mut word)?;
    word = word.replace('\n', "");

    Ok(word)
}
