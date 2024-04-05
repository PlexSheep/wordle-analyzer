use std::sync::{Arc, Mutex};

use libpt::log::info;

use crate::error::{BenchError, Error, WResult};
use crate::game::{Game, GameBuilder};
use crate::solve::Solver;
use crate::wlist::WordList;

use super::{Benchmark, Report};

#[derive(Debug, Clone)]
pub struct BuiltinBenchmark<'wl, WL: WordList, SL: Solver<'wl, WL>> {
    wordlist: &'wl WL,
    solver: SL,
    builder: GameBuilder<'wl, WL>,
    report: Arc<Mutex<Report>>,
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
        threads: usize
    ) -> crate::error::WResult<Self> {
        info!("using {threads} threads for benchmarking");
        rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().unwrap();
        Ok(Self {
            wordlist,
            solver,
            report: Arc::new(Mutex::new(Report::new(builder.build()?))),
            builder,
        })
    }
    fn solver(&self) -> &SL {
        &self.solver
    }
    fn builder(&'wl self) -> &'wl crate::game::GameBuilder<'wl, WL> {
        &self.builder
    }

    fn report_mutex(&'wl self) -> Arc<Mutex<Report>> {
        self.report.clone()
    }

    fn report(&'wl self) -> super::Report {
        self.report.lock().expect("lock is poisoned").clone()
    }
}
