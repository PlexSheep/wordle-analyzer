use crate::game::{Game, GameBuilder};
use crate::solve::Solver;
use crate::wlist::WordList;

use super::Benchmark;

#[derive(Debug, Clone)]
pub struct BuiltinBenchmark<'wl, WL: WordList, SL: Solver<'wl, WL>> {
    wordlist: &'wl WL,
    solver: SL,
    builder: GameBuilder<'wl, WL>,
}

impl<'wl, WL: WordList, SL: Solver<'wl, WL>> Benchmark<'wl, WL, SL>
    for BuiltinBenchmark<'wl, WL, SL>
{
    fn build(wordlist: &'wl WL, solver: SL) -> crate::error::WResult<Self> {
        let builder: GameBuilder<_> = Game::builder(wordlist);
        Ok(Self {
            wordlist,
            solver,
            builder,
        })
    }
    fn solver(&self) -> &SL {
        &self.solver
    }
    fn builder(&'wl self) -> &'wl crate::game::GameBuilder<'wl, WL> {
        &self.builder
    }
}
