use libpt::log::info;

use crate::wlist::word::{ManyWordDatas, Word};
use crate::wlist::WordList;

use super::{AnyBuiltinSolver, Solver, Status};

#[derive(Debug, Clone)]
pub struct NaiveSolver<'wl, WL> {
    wl: &'wl WL,
}

impl<'wl, WL: WordList> Solver<'wl, WL> for NaiveSolver<'wl, WL> {
    fn build(wordlist: &'wl WL) -> crate::error::WResult<Self> {
        info!("using naive solver");
        Ok(Self { wl: wordlist })
    }
    fn guess_for(&self, game: &crate::game::Game<WL>) -> Word {
        // HACK: hardcoded length
        let mut buf: Word = Word::from(".....");
        let response = game.last_response();
        if response.is_some() {
            for (idx, p) in response.unwrap().evaluation().iter().enumerate() {
                if p.1 == Status::Matched {
                    buf.replace_range(idx..idx + 1, &p.0.to_string());
                }
            }
        }
        let matches: ManyWordDatas = game
            .wordlist()
            .get_words_matching(buf)
            .expect("the solution does not exist in the wordlist")
            .iter()
            .filter(|m| !game.made_guesses().contains(&&m.0))
            .map(|v| v.to_owned())
            .collect();
        matches[0].0.to_owned()
    }
}

impl<'wl, WL: WordList> From<NaiveSolver<'wl, WL>> for AnyBuiltinSolver<'wl, WL> {
    fn from(value: NaiveSolver<'wl, WL>) -> Self {
        Self::Naive(value)
    }
}
