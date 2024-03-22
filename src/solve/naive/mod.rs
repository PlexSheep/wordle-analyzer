use libpt::log::info;

use crate::wlist::word::Word;
use crate::wlist::WordList;

use super::{AnyBuiltinSolver, Solver};

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
        self.wl.rand_word().0
    }
}

impl<'wl, WL: WordList> From<NaiveSolver<'wl, WL>> for AnyBuiltinSolver<'wl, WL> {
    fn from(value: NaiveSolver<'wl, WL>) -> Self {
        Self::Naive(value)
    }
}
