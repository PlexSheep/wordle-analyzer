use chrono::{self, NaiveDateTime, NaiveTime, TimeDelta};
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::game::response::GuessResponse;

#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    data: Vec<GuessResponse>,
    start: NaiveDateTime,
    end: Option<NaiveDateTime>,
    benchtime: Option<TimeDelta>,
    /// is the benchmark finished?
    finished: bool,
}

impl Report {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            start: chrono::Local::now().naive_local(),
            benchtime: None,
            end: None,
            finished: false,
        }
    }
    pub fn add(&mut self, data: GuessResponse) {
        self.data.push(data)
    }

    pub fn n(&self) -> usize {
        self.data.len()
    }

    pub fn win_ratio(&self) -> usize {
        todo!()
    }

    pub fn avg_score(&self) -> usize {
        todo!()
    }

    /// finalize the record
    ///
    /// Sets the [benchtime](Report::benchtime) and [over](Report::over). In future versions, this
    /// method might be used to precompute statistical information from the data.
    pub(crate) fn finalize(&mut self) {
        self.end = Some(chrono::Local::now().naive_local());
        self.benchtime = Some(self.end.unwrap() - self.start);
        self.finished = true;
    }

    /// is the report finished?
    ///
    /// Will be true after the [benchmark][super::Benchmark] is done.
    pub fn finished(&self) -> bool {
        self.finished
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "n: {}, win_ratio: {}, avg_score: {}",
            self.n(),
            self.win_ratio(),
            self.avg_score()
        )
    }
}
