use std::fmt::{Debug, Display};

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

/// Default amount of games to play for a [Benchmark]
pub const DEFAULT_N: usize = 50;

pub trait Benchmark<'wl, WL, SL>: Clone + Sized + Debug
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
    ) -> crate::error::WResult<Self>;
    fn builder(&'wl self) -> &'wl GameBuilder<'wl, WL>;
    fn make_game(&'wl self) -> WResult<Game<'wl, WL>> {
        Ok(self.builder().build()?)
    }
    fn solver(&'wl self) -> &'wl SL;
    fn play(&'wl self) -> WResult<GuessResponse> {
        self.solver().play(&mut self.make_game()?)
    }
    // TODO: add some interface to get reports while the benchmark runs
    // TODO: make the benchmark optionally multithreaded
    fn bench(&'wl self, n: usize) -> WResult<Report> {
        // PERF: it would be better to make this multithreaded
        let part = match n / 20 {
            0 => 19,
            other => other,
        };
        let mut report = Report::new();

        for i in 0..n {
            report.add(self.play()?);
            if i % part == part - 1 {
                // TODO: add the report to the struct so that users can poll it to print the status
                // TODO: update the report in the struct
            }
        }

        report.finalize();

        Ok(report)
    }
}
