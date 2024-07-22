use crate::wlist::word::{Word, WordData};
use crate::wlist::WordList;
use colored::Colorize;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fmt::Display;
use std::str::FromStr;

use super::Game;

#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AtomicEvaluation {
    char: char,
    status: Status,
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Evaluation {
    inner: Vec<EvaluationUnit>,
}
pub type EvaluationUnit = (char, Status);

impl IntoIterator for Evaluation {
    type Item = EvaluationUnit;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl From<Vec<EvaluationUnit>> for Evaluation {
    fn from(value: Vec<EvaluationUnit>) -> Self {
        Self { inner: value }
    }
}

impl FromStr for Evaluation {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: make this proper
        Ok(vec![('x', Status::None)].into())
    }
}

#[derive(Debug, Clone, PartialEq)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GuessResponse {
    guess: Word,
    evaluation: Evaluation,
    finish: bool,
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

impl GuessResponse {
    pub(crate) fn new<WL: WordList>(guess: &Word, status: Evaluation, game: &Game<WL>) -> Self {
        let new = Self {
            guess: guess.to_owned(),
            evaluation: status,
            finish: game.step() > game.max_steps(),
            solution: game.solution().cloned(),
            step: game.step(),
            max_steps: game.max_steps(),
        };
        new
    }

    pub fn finished(&self) -> bool {
        self.finish
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
        for s in self.evaluation.clone().into_iter() {
            write!(
                f,
                "{}",
                match s.1 {
                    Status::None => s.0.to_string().into(),
                    Status::Exists => s.0.to_string().yellow(),
                    Status::Matched => s.0.to_string().green(),
                }
            )?;
        }
        std::fmt::Result::Ok(())
    }
}
