use wordle_analyzer::error::GameError;
use wordle_analyzer::game::evaluation::Evaluation;
use wordle_analyzer::game::response::GuessResponse;
use wordle_analyzer::wlist::builtin::BuiltinWList;
use wordle_analyzer::wlist::WordList;

use wordle_analyzer::wlist::word::Word;
use wordle_analyzer::{self, game};

fn wordlist() -> impl WordList {
    BuiltinWList::default()
}

#[test]
fn test_eval_simple() -> anyhow::Result<()> {
    let wl = wordlist();
    let builder = game::Game::builder(&wl)
        .length(5)
        .max_steps(6)
        .solution(Some(wl.get_word(&Word::from("crate")).unwrap()))
        .precompute(false);

    let mut game = builder.build()?;
    let guess = Word::from("slate");
    game.guess(&guess, None)?;
    assert_eq!(
        *game.last_response().unwrap().evaluation(),
        Evaluation::build(&guess, "xxccc")?
    );
    Ok(())
}

#[test]
fn test_eval_reoccuring_char() -> anyhow::Result<()> {
    let wl = wordlist();
    let builder = game::Game::builder(&wl)
        .solution(Some(wl.get_word(&Word::from("nines")).unwrap()))
        .precompute(false);

    let mut game = builder.build()?;
    let guess = Word::from("pines");
    game.guess(&guess, None)?;
    assert_eq!(
        *game.last_response().unwrap().evaluation(),
        Evaluation::build(&guess, "xcccc")?
    );

    let mut game = builder.build()?;
    let guess = Word::from("sides");
    game.guess(&guess, None)?;
    assert_eq!(
        *game.last_response().unwrap().evaluation(),
        Evaluation::build(&guess, "xcxcc")?
    );

    Ok(())
}
