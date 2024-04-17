use std::sync::{Arc, RwLock};

use libpt::log::info;

use crate::game::{self, GameBuilder};
use crate::solve::Solver;
use crate::wlist::WordList;

use super::{Benchmark, Report};

#[derive(Debug, Clone)]
pub struct BuiltinBenchmark<'wl, WL: WordList, SL: Solver<'wl, WL>> {
    solver: SL,
    builder: GameBuilder<'wl, WL>,
    report: Arc<RwLock<Report>>,
    finished: bool
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
            finished: false
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
    fn is_finished(&self) -> bool {
        self.finished
    }
}
