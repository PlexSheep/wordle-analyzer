use crate::wlist::word::Word;
use anyhow::Ok;
use colored::{ColoredString, Colorize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuessResponse {
    guess: Word,
    evaluation: Vec<(char, Status)>,
    step: usize,
    finish: bool,
    win: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    None = 0,
    Exists = 1,
    Matched = 2,
}

impl GuessResponse {
    pub(crate) fn new(
        guess: Word,
        status: Vec<(char, Status)>,
        step: usize,
        max_step: usize,
    ) -> Self {
        let mut win = false;
        let mut finish: bool = if step >= max_step {
            true
        } else {
            let mut matched = true;
            for p in &status {
                matched &= p.1 == Status::Matched;
            }
            win = matched;
            win
        };
        Self {
            guess,
            evaluation: status,
            step,
            finish,
            win,
        }
    }

    pub fn finished(&self) -> bool {
        self.finish
    }

    pub fn won(&self) -> bool {
        self.win
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
