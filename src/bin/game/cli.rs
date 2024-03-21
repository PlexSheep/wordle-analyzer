use clap::Parser;
use libpt::log::*;
use wordle_analyzer::{game,self};

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
    max_steps: usize
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    Logger::build_mini(Some(Level::TRACE))?;
    debug!("dumping CLI: {:#?}", cli);

    let game = game::Game::builder()
        .length(cli.length)
        .precompute(cli.precompute).build()?;

    Ok(())
}

fn get_word(cli: &Cli) -> String {
    let mut word = String::new();

    todo!("get user input");
    todo!("validate user input");

    assert_eq!(word.len(), cli.length);
    word
}
