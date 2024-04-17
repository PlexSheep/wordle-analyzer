use std::sync::{Arc, Mutex, RwLock};
use std::thread::JoinHandle;

use libpt::log::info;

use crate::error::{BenchError, Error, WResult};
use crate::game::{self, Game, GameBuilder};
use crate::solve::Solver;
use crate::wlist::WordList;

use super::{Benchmark, Report};

use rayon::prelude::*;

#[derive(Debug)]
pub struct BuiltinBenchmark<'wl, WL: WordList, SL: Solver<'wl, WL>> {
    solver: SL,
    builder: GameBuilder<'wl, WL>,
    report: Arc<RwLock<Report>>,
    benchth: Option<JoinHandle<()>>,
}

impl<'wl, WL, SL> Benchmark<'wl, WL, SL> for BuiltinBenchmark<'wl, WL, SL>
where
    WL: WordList,
    WL: 'wl,
    SL: Solver<'wl, WL>,
    SL: 'wl,
    SL: Send,
{
    fn build(
        _wordlist: &'wl WL,
        solver: SL,
        builder: GameBuilder<'wl, WL>,
        threads: usize,
    ) -> crate::error::WResult<Self> {
        info!("using {threads} threads for benchmarking");
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .unwrap();
        Ok(Self {
            solver,
            report: Arc::new(RwLock::new(Report::new(builder.build()?))),
            builder,
            benchth: None,
        })
    }
    fn solver(&self) -> SL {
        self.solver.clone()
    }
    fn builder(&'wl self) -> game::GameBuilder<'wl, WL> {
        self.builder.clone()
    }
    fn solver_ref(&'wl self) -> &'wl SL {
        &self.solver
    }
    fn builder_ref(&'wl self) -> &'wl game::GameBuilder<'wl, WL> {
        &self.builder
    }

    fn report_shared(&'wl self) -> Arc<RwLock<Report>> {
        self.report.clone()
    }

    fn report(&'wl self) -> super::Report {
        self.report.read().expect("lock is poisoned").clone()
    }

    fn start(&'wl self, n: usize) -> WResult<()> {
        let report = self.report_shared(); // FIXME: needs to keep self borrowed for some reason?
        let solver = self.solver();
        let builder = self.builder();
        let benchth = std::thread::spawn({
            move || {
                // TODO: do the stuff
                report.write().expect("lock is poisoned").finalize();
            }
        });

        // self.benchth = Some(benchth);
        Ok(())
    }

    fn is_finished(&self) -> Option<bool> {
        match &self.benchth {
            Some(th) => Some(th.is_finished()),
            None => None,
        }
    }
}
