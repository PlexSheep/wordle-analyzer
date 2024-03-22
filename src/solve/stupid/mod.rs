use crate::wlist::word::Word;
use crate::wlist::WordList;

use super::Solver;

#[derive(Debug, Clone)]
pub struct StupidSolver<'wl, WL> {
    wl: &'wl WL,
}

impl<'wl, WL: WordList> Solver<'wl, WL> for StupidSolver<'wl, WL> {
    fn build(wordlist: &'wl WL) -> crate::error::WResult<Self> {
        Ok(Self { wl: wordlist })
    }
    fn guess_for(&self, game: &crate::game::Game<WL>) -> Word {
        self.wl.rand_word().0
    }
}
