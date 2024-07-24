use std::convert::Infallible;
use std::str::FromStr;

use libpt::cli::console::{style, StyledObject};

use crate::wlist::word::Word;

use super::response::Status;

pub type EvaluationUnit = (char, Status);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Evaluation {
    inner: Vec<EvaluationUnit>,
}

impl Evaluation {
    pub(crate) fn colorized_display(&self, guess: &Word) -> Vec<StyledObject<String>> {
        assert_eq!(guess.len(), self.inner.len());
        let mut buf = Vec::new();
        for (i, e) in self.inner.iter().enumerate() {
            let mut c = style(guess.chars().nth(i).unwrap().to_string());
            if e.1 == Status::Matched {
                c = c.green();
            } else if e.1 == Status::Exists {
                c = c.yellow();
            }
            buf.push(c);
        }
        buf
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

impl From<&str> for Evaluation {
    fn from(value: &str) -> Self {
        Self::from_str(value).unwrap()
    }
}

impl FromStr for Evaluation {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v: Vec<EvaluationUnit> = Vec::new();
        for c in s.chars() {
            v.push((c, Status::from(c)))
        }
        Ok(v.into())
    }
}
