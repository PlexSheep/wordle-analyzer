use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use libpt::log::debug;
use rayon::prelude::*;

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

pub trait Benchmark<'wl, WL, SL>: Clone + Sized + Debug + Sync
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
        let part = match n / 20 {
            0 => 19,
            other => other,
        };
        let report = Arc::new(Mutex::new(Report::new()));
        let this = std::sync::Arc::new(self);

        (0..n)
            .into_par_iter()
            .for_each_with(report.clone(), |outside_data, _i| {
                let report = outside_data;
                let r = this
                    .play()
                    .expect("error playing the game during benchmark");
                report.lock().expect("lock is poisoned").add(r);
            });

        // FIXME: find some way to take the Report from the Mutex
        // Mutex::into_inner() does not work
        let mut report: Report = report.lock().unwrap().clone();
        report.finalize();

        Ok(report)
    }
}
