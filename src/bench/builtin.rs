use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::JoinHandle;

use libpt::log::info;

use crate::error::WResult;
use crate::game::{self, GameBuilder};
use crate::solve::Solver;
use crate::wlist::WordList;

use super::{Benchmark, Report};

#[derive(Debug)]
pub struct BuiltinBenchmark<'wl, WL: WordList, SL: Solver<'wl, WL>> {
    solver: SL,
    builder: GameBuilder<'wl, WL>,
    report: Arc<RwLock<Report>>,
    finished: AtomicBool,
    bench_th: Arc<Mutex<Option<JoinHandle<WResult<Report>>>>> // HACK: this is unholy
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
        Ok(Self {
            solver,
            report: Arc::new(RwLock::new(Report::new(builder.build()?))),
            builder,
            finished: AtomicBool::new(false),
            bench_th: Arc::new(Mutex::new(None))
        })
    }
    #[inline]
    fn solver(&self) -> SL {
        self.solver.clone()
    }
    #[inline]
    fn builder(&'wl self) -> game::GameBuilder<'wl, WL> {
        self.builder.clone()
    }
    #[inline]
    fn solver_ref(&'wl self) -> &'wl SL {
        &self.solver
    }
    #[inline]
    fn builder_ref(&'wl self) -> &'wl game::GameBuilder<'wl, WL> {
        &self.builder
    }

    #[inline]
    fn report_shared(&'wl self) -> Arc<RwLock<Report>> {
        self.report.clone()
    }

    #[inline]
    fn report(&'wl self) -> super::Report {
        self.report.read().expect("lock is poisoned").clone()
    }
    fn is_finished(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Relaxed)
    }
    fn start(&'wl self, n: usize) -> WResult<()> {
        let report = self.report_shared();
        let solver = self.solver();
        let builder = self.builder();
        let th = std::thread::spawn(move||{
            Self::bench(n, report, solver, builder)
        });
        *self.bench_th.lock().expect("lock is poisoned") = Some(th);
        Ok(())
    }
}
