use chrono::{self, Duration, NaiveDateTime, NaiveTime, TimeDelta};
use libpt::log::debug;
use core::panic;
use std::fmt::Display;
use std::ops::Div;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::game::response::GuessResponse;

pub const WEIGHTING_SCORE: f64 = 0.9;
pub const WEIGHTING_TIME: f64 = 0.1;
pub const WEIGHTING_WIN: f64 = 0.0;

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

    pub fn total_wins(&self) -> usize {
        let mut wins: usize = 0;
        self.data.iter().for_each(|d| {
            if d.won() {
                wins += 1;
            }
        });
        wins
    }

    pub fn avg_win(&self) -> f64 {
        self.total_wins() as f64 / self.n() as f64
    }

    pub fn total_score(&self) -> usize {
        let mut score: usize = 0;
        self.data.iter().for_each(|d| score += d.step());
        score
    }

    pub fn avg_score(&self) -> f64 {
        self.total_score() as f64 / self.n() as f64
    }

    pub fn avg_time(&self) -> Option<TimeDelta> {
        let av = self.benchtime()? / self.n() as i32;
        Some(av)
    }

    pub fn rating(&self) -> Option<f64> {
        assert_eq!(WEIGHTING_WIN + WEIGHTING_SCORE + WEIGHTING_TIME, 1.0);
        debug!(
            "partial rating - score: {}",
            WEIGHTING_SCORE * self.avg_score()
        );
        debug!(
            "partial rating - win: {}",
            WEIGHTING_WIN * (1.0 - self.avg_win())
        );
        // FIXME: this is not normalized, which can lead to negative score
        debug!(
            "partial rating - time: {}",
            WEIGHTING_TIME
                * (1.0
                    - (1_000_000_000.0
                        / self
                            .avg_time()?
                            .num_nanoseconds()
                            .expect("took so many ns per game that an integer overflow occured")
                            as f64))
        );
        let r = WEIGHTING_SCORE * self.avg_score()
            + WEIGHTING_WIN * (1.0 - self.avg_win())
            + WEIGHTING_TIME
                * (1.0
                    - (1_000_000_000.0
                        / self
                            .avg_time()?
                            .num_nanoseconds()
                            .expect("took so many ns per game that an integer overflow occured")
                            as f64));
        Some(r)
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

    pub fn benchtime(&self) -> Option<TimeDelta> {
        self.benchtime
    }
}

impl Display for Report {
    /// Implement the [Display] trait
    ///
    /// # Panics
    ///
    /// This will panic if the [Report] is not finished
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.finished {
            panic!("can only display finished reports");
        }
        write!(
            f,
            "n: {}, win_ratio: {:.2}%, avg_score: {:.4} steps until finish, avgerage time per game: {}μs, \n\
            rating: {:.4}, full time until completion: {}ms
            ",
            self.n(),
            self.avg_win() * 100.0,
            self.avg_score(),
            self.avg_time().unwrap().num_microseconds().expect("overflow when converting to micrseconds"),
            self.rating().unwrap(),
            self.benchtime().unwrap().num_milliseconds()
        )
    }
}
