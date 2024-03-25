use std::fmt::Debug;

use libpt::log::debug;

use crate::error::WResult;
use crate::wlist::WordList;

pub trait Benchmark<'wl, WL: WordList>: Clone + Sized + Debug {
    fn build(wordlist: &'wl WL) -> WResult<Self>;
    fn play(&self) -> WResult<usize>;
    fn bench(&self, n: usize) -> WResult<(usize, f64)> {
        // PERF: it would be better to make this multithreaded
        let mut absolute: usize = 0;
        let part = n / 20;

        let start = std::time::Instant::now();

        for i in 0..n {
            // TODO: limit execution time for the following, perhaps async
            absolute += self.play()?;
            if i % part == part - 1 {
                debug!(
                    "{} / {n}\t ratio: {} \t elapsed: {:?}",
                    i + 1,
                    absolute as f64 / n as f64,
                    start.elapsed()
                );
            }
        }

        Ok((absolute, absolute as f64 / n as f64))
    }
}
