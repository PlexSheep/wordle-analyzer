use crate::wlist::word::{Word, WordData};
use crate::wlist::WordList;
use anyhow::Ok;
use colored::{ColoredString, Colorize};
use libpt::log::debug;
use std::fmt::Display;

use super::Game;

pub type Evaluation = Vec<(char, Status)>;

#[derive(Debug, Clone, PartialEq)]
pub struct GuessResponse {
    guess: Word,
    evaluation: Evaluation,
    finish: bool,
    solution: WordData,
    step: usize,
    max_steps: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    None = 0,
    Exists = 1,
    Matched = 2,
}

impl GuessResponse {
    pub(crate) fn new<WL: WordList>(
        guess: Word,
        status: Vec<(char, Status)>,
        game: &Game<WL>,
    ) -> Self {
        let finish: bool = if game.step() > game.max_steps() {
            true
        } else {
            guess == game.solution().0
        };
        Self {
            guess,
            evaluation: status,
            finish,
            solution: game.solution().clone(),
            step: game.step(),
            max_steps: game.max_steps(),
        }
    }

    pub fn finished(&self) -> bool {
        self.finish
    }

    pub fn won(&self) -> bool {
        self.guess == self.solution.0
    }

    pub fn solution(&self) -> Option<WordData> {
        if self.won() {
            Some(self.solution.clone())
        } else {
            None
        }
    }

    pub fn evaluation(&self) -> &[(char, Status)] {
        &self.evaluation
    }

    pub fn guess(&self) -> &Word {
        &self.guess
    }
}

impl Display for GuessResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in &self.evaluation {
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
