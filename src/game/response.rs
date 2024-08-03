use crate::wlist::word::{Word, WordData};
use crate::wlist::WordList;
use colored::Colorize;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::{Evaluation, Game};

#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AtomicEvaluation {
    char: char,
    status: Status,
}

#[derive(Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GuessResponse {
    guess: Word,
    evaluation: Evaluation,
    solution: Option<WordData>,
    step: usize,
    max_steps: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Status {
    None = 0,
    Exists = 1,
    Matched = 2,
}

impl From<char> for Status {
    fn from(value: char) -> Self {
        let value = value.to_ascii_lowercase(); // let's not deal with unicode here, wordle is
                                                // ASCII centric anyway
        match value {
            'x' => Self::None,
            'f' | 'e' => Self::Exists,
            'c' | 'm' => Self::Matched,
            _ => Self::None,
        }
    }
}

impl GuessResponse {
    pub(crate) fn new<WL: WordList>(guess: &Word, status: Evaluation, game: &Game<WL>) -> Self {
        let new = Self {
            guess: guess.to_owned(),
            evaluation: status,
            solution: game.solution().cloned(),
            step: game.step(),
            max_steps: game.max_steps(),
        };
        new
    }

    pub fn finished(&self) -> bool {
        self.step() > self.max_steps() || self.won()
    }

    pub fn won(&self) -> bool {
        let mut ok = true;
        for i in self.evaluation.clone().into_iter() {
            ok &= i.1 == Status::Matched
        }
        ok
    }

    pub fn solution(&self) -> Option<WordData> {
        self.solution.clone()
    }

    pub fn evaluation(&self) -> &Evaluation {
        &self.evaluation
    }

    pub fn guess(&self) -> &Word {
        &self.guess
    }

    pub fn step(&self) -> usize {
        self.step
    }

    pub fn max_steps(&self) -> usize {
        self.max_steps
    }
}

impl Display for GuessResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.evaluation())
    }
}
