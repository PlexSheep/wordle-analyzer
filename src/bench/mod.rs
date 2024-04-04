use std::fmt::Debug;

use libpt::log::debug;

use crate::error::WResult;
use crate::game::response::GuessResponse;
use crate::game::{Game, GameBuilder};
use crate::solve::Solver;
use crate::wlist::WordList;

pub mod report;
use report::*;

#[cfg(feature = "builtin")]
pub mod builtin;

pub trait Benchmark<'wl, WL, SL>: Clone + Sized + Debug
where
    WL: WordList,
    WL: 'wl,
    SL: Solver<'wl, WL>,
    SL: 'wl,
{
    fn build(wordlist: &'wl WL, solver: SL) -> WResult<Self>;
    fn builder(&'wl self) -> &'wl GameBuilder<'wl, WL>;
    fn make_game(&'wl self) -> WResult<Game<'wl, WL>> {
        Ok(self.builder().build()?)
    }
    fn solver(&'wl self) -> &'wl SL;
    fn play(&'wl self) -> WResult<GuessResponse> {
        self.solver().play(&mut self.make_game()?)
    }
    fn bench(&'wl self, n: usize) -> WResult<Report> {
        // PERF: it would be better to make this multithreaded
        let part = n / 20;
        let mut report = Report::new();

        for i in 0..n {
            // TODO: limit execution time for the following, perhaps async
            report.add(self.play()?);
            if i % part == part - 1 {
                debug!("{}", report);
            }
        }

        report.finalize();

        Ok(report)
    }
}
