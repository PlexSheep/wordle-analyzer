use std::fmt::Debug;
use std::sync::{Arc, RwLock};

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

pub trait Benchmark<'wl, WL, SL>: Sized + Debug + Sync
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
        threads: usize,
    ) -> crate::error::WResult<Self>;
    fn builder(&'wl self) -> GameBuilder<'wl, WL>;
    fn builder_ref(&'wl self) -> &'wl GameBuilder<'wl, WL>;
    fn make_game(&'wl self) -> WResult<Game<'wl, WL>> {
        Ok(self.builder_ref().build()?)
    }
    fn solver(&'wl self) -> SL;
    fn solver_ref(&'wl self) -> &'wl SL;
    fn play(&'wl self) -> WResult<GuessResponse> {
        self.solver_ref().play(&mut self.make_game()?)
    }
    // TODO: add some interface to get reports while the benchmark runs
    // TODO: make the benchmark optionally multithreaded
    // NOTE: This is blocking, use start to let it run in another thread
    // FIXME: this never stops? Reports just keep getting printed
    fn bench(
        &self,
        n: usize,
        report: Arc<RwLock<Report>>,
        solver: SL,
        builder: &'wl GameBuilder<'wl, WL>,
    ) -> WResult<Report> {
        (0..n)
            .into_par_iter()
            .for_each_with(report.clone(), |outside_data, _i| {
                let report = outside_data;
                let r = solver
                    .play_owned(builder.build().expect("could not create game"))
                    .expect("error playing the game during benchmark");
                report.write().expect("lock is poisoned").add(r);
            });
        libpt::log::info!("finished playing games, finalizing report");
        report.write().expect("lock is poisoned").finalize();
        debug!("finalized the report");
        self.set_finished(true)?;
        Ok(report.read().expect("lock is poisoned").clone())
    }
    // PERF: Somehow returning &Report would be better as we don't need to clone then
    fn report(&'wl self) -> Report;
    fn report_shared(&'wl self) -> Arc<RwLock<Report>>;
    fn start(&'wl self, n: usize, builder: &'wl GameBuilder<'wl, WL>) -> WResult<()>;
    fn is_finished(&self) -> bool;
    fn set_finished(&self, value: bool) -> WResult<()>;
}
