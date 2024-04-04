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

impl<'wl, WL, SL> Benchmark<'wl, WL, SL> for BuiltinBenchmark<'wl, WL, SL>
where
    WL: WordList,
    WL: 'wl,
    SL: Solver<'wl, WL>,
    SL: 'wl,
{
    fn build(
        wordlist: &'wl WL,
        solver: SL,
        builder: GameBuilder<'wl, WL>,
    ) -> crate::error::WResult<Self> {
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
