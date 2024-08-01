use libpt::cli::console::{style, StyledObject};

use crate::wlist::word::Word;

use super::response::Status;
use super::{GameError, WResult};

/// the [char] of the guess and the [Status] associated with it
pub type EvaluationUnit = (char, Status);

/// Basically a [String] with extra information associated with each char
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Evaluation {
    inner: Vec<EvaluationUnit>,
}

impl Evaluation {
    /// Display the evaluation color coded
    pub fn colorized_display(&self) -> Vec<StyledObject<String>> {
        let mut buf = Vec::new();
        for e in self.inner.iter() {
            let mut c = style(e.0.to_string());
            if e.1 == Status::Matched {
                c = c.green();
            } else if e.1 == Status::Exists {
                c = c.yellow();
            }
            buf.push(c);
        }
        buf
    }

    /// The first string is the word the evaluation is for, The second string defines how the
    /// characters of the first string match the solution.
    ///
    ///
    /// ## Evaluation Format:
    ///
    /// 'x' means wrong character
    ///
    /// 'p' means present character
    ///
    /// 'c' means correct character
    ///
    /// ### Example:
    ///
    /// 'xxxcc' --- means the first 3 chars are wrong but the second 2 chars are correct
    ///
    /// 'xppxc' --- means the first character is wrong, the next two characters are present, the last
    /// is correct
    ///
    /// ## Example
    ///
    /// "wordle xxxcff" --- the guess was wordle, the d is in the correct spot, the solution
    /// contains 'l' and 'e', but on another index.
    ///
    pub fn build(guess: &Word, eval_str: &str) -> WResult<Self> {
        if guess.len() != eval_str.len() {
            return Err(GameError::GuessAndEvalNotSameLen((
                guess.to_string(),
                eval_str.to_string(),
            ))
            .into());
        }
        let mut v: Vec<EvaluationUnit> = Vec::new();
        for (c, e) in guess.chars().zip(eval_str.chars()) {
            v.push((c, Status::from(e)))
        }
        Ok(v.into())
    }

    pub fn inner(&self) -> &Vec<EvaluationUnit> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Vec<EvaluationUnit> {
        &mut self.inner
    }

    pub fn guess(&self) -> Word {
        Word::from(self)
    }
}

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

impl From<Evaluation> for Word {
    fn from(value: Evaluation) -> Self {
        Word::from(value.inner.iter().map(|v| v.0).collect::<String>())
    }
}

impl From<&Evaluation> for Word {
    fn from(value: &Evaluation) -> Self {
        Word::from(value.inner.iter().map(|v| v.0).collect::<String>())
    }
}
