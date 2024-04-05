use chrono::{self, Duration, NaiveDateTime, NaiveTime, TimeDelta};
use core::panic;
use libpt::log::debug;
use std::fmt::Display;
use std::ops::Div;
use rayon::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::game::response::GuessResponse;
use crate::game::Game;
use crate::wlist::WordList;

pub const WEIGHTING_STEPS: f64 = 1000.0;
pub const WEIGHTING_TIME: f64 = 50.0;
pub const WEIGHTING_WIN: f64 = 1000.0;

#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    data: Vec<GuessResponse>,
    start: NaiveDateTime,
    end: Option<NaiveDateTime>,
    benchtime: Option<TimeDelta>,
    /// is the benchmark finished?
    finished: bool,
    max_steps: usize,
}

impl Report {
    pub fn new<WL: WordList>(example_game: Game<'_, WL>) -> Self {
        Self {
            data: Vec::new(),
            start: chrono::Local::now().naive_local(),
            benchtime: None,
            end: None,
            finished: false,
            max_steps: example_game.max_steps(),
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

    pub fn total_steps(&self) -> usize {
        let mut steps: usize = 0;
        self.data.iter().for_each(|d| steps += d.step() - 1);
        steps
    }

    pub fn avg_steps(&self) -> f64 {
        self.total_steps() as f64 / self.n() as f64
    }

    pub fn avg_time(&self) -> TimeDelta {
        if self.n() == 0 {
            return TimeDelta::new(0, 0).unwrap();
        }
        self.benchtime() / self.n() as i32
    }

    fn rating_steps(&self) -> f64 {
        let n = self.avg_steps() / self.max_steps() as f64;
        WEIGHTING_STEPS * n
    }

    fn rating_win(&self) -> f64 {
        WEIGHTING_WIN * (1.0 - self.avg_win())
    }

    fn rating_time(&self) -> f64 {
        let n = 1.0
            / (1.0
                + (self
                    .avg_time()
                    .num_nanoseconds()
                    .expect("nanoseconds overflow") as f64)
                    .exp());
        WEIGHTING_TIME * (1.0 - n)
    }

    pub fn rating(&self) -> f64 {
        let rating_steps: f64 = self.rating_steps();
        let rating_win: f64 = self.rating_win();
        let rating_time: f64 = self.rating_time();
        debug!("partial rating - steps: {}", rating_steps);
        debug!("partial rating - win: {}", rating_win);
        debug!("partial rating - time: {:?}", rating_time);
        rating_win + rating_time + rating_steps
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

    pub fn benchtime(&self) -> TimeDelta {
        chrono::Local::now().naive_local() - self.start
    }

    pub fn max_steps(&self) -> usize {
        self.max_steps
    }
}

impl Display for Report {
    /// Implement the [Display] trait
    ///
    /// # Panics
    ///
    /// This will panic if the [Report] is not finished
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "n: {}, win_ratio: {:.2}%, avg_score: {:.4} steps until finish, avgerage time per game: {}μs, \n\
            rating: {:.4}, full time until completion: {}ms
            ",
            self.n(),
            self.avg_win() * 100.0,
            self.avg_steps(),
            self.avg_time(),
            self.rating(),
            self.benchtime()
        )
    }
}
