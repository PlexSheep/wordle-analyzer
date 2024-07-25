use libpt::log::info;

use crate::error::WResult;
use crate::wlist::word::Word;
use crate::wlist::WordList;

use super::{AnyBuiltinSolver, Solver};

#[derive(Debug, Clone)]
pub struct StupidSolver<'wl, WL> {
    wl: &'wl WL,
}

impl<'wl, WL: WordList> Solver<'wl, WL> for StupidSolver<'wl, WL> {
    fn build(wordlist: &'wl WL) -> crate::error::WResult<Self> {
        info!("using stupid solver");
        Ok(Self { wl: wordlist })
    }
    fn guess_for(&self, game: &crate::game::Game<WL>) -> WResult<Word> {
        Ok(self.wl.rand_word().0)
    }
}

impl<'wl, WL: WordList> From<StupidSolver<'wl, WL>> for AnyBuiltinSolver<'wl, WL> {
    fn from(value: StupidSolver<'wl, WL>) -> Self {
        Self::Stupid(value)
    }
}
