use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::game::response::GuessResponse;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Report {
    data: Vec<GuessResponse>,
}

impl Report {
    pub fn new() -> Self {
        Self { data: Vec::new() }
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
