use test_log::test; // set the log level with an envvar: `RUST_LOG=trace cargo test`

use libpt::log::info;
use wordle_analyzer::game::evaluation::Evaluation;
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
    let correct = Evaluation::build(&guess, "xxccc")?;
    info!(
        "{} =? {}",
        *game.last_response().unwrap().evaluation(),
        correct
    );
    assert_eq!(*game.last_response().unwrap().evaluation(), correct);

    let mut game = builder.build()?;
    let guess = Word::from("about");
    game.guess(&guess, None)?;
    let correct = Evaluation::build(&guess, "fxxxf")?;
    info!(
        "{} =? {}",
        *game.last_response().unwrap().evaluation(),
        correct
    );
    assert_eq!(*game.last_response().unwrap().evaluation(), correct);

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
    let correct = Evaluation::build(&guess, "xcccc")?;
    info!(
        "{} =? {}",
        *game.last_response().unwrap().evaluation(),
        correct
    );
    assert_eq!(*game.last_response().unwrap().evaluation(), correct);

    let mut game = builder.build()?;
    let guess = Word::from("sides");
    game.guess(&guess, None)?;
    let correct = Evaluation::build(&guess, "xcxcc")?;
    info!(
        "{} =? {}",
        *game.last_response().unwrap().evaluation(),
        correct
    );
    assert_eq!(*game.last_response().unwrap().evaluation(), correct);

    let mut game = builder.build()?;
    let guess = Word::from("ninja");
    game.guess(&guess, None)?;
    let correct = Evaluation::build(&guess, "cccxx")?;
    info!(
        "{} =? {}",
        *game.last_response().unwrap().evaluation(),
        correct
    );
    assert_eq!(*game.last_response().unwrap().evaluation(), correct);

    let mut game = builder.build()?;
    let guess = Word::from("which");
    game.guess(&guess, None)?;
    let correct = Evaluation::build(&guess, "xxfxx")?;
    info!(
        "{} =? {}",
        *game.last_response().unwrap().evaluation(),
        correct
    );
    assert_eq!(*game.last_response().unwrap().evaluation(), correct);

    let mut game = builder.build()?;
    let guess = Word::from("indie");
    game.guess(&guess, None)?;
    let correct = Evaluation::build(&guess, "ffxxf")?;
    info!(
        "{} =? {}",
        *game.last_response().unwrap().evaluation(),
        correct
    );
    assert_eq!(*game.last_response().unwrap().evaluation(), correct);

    Ok(())
}
