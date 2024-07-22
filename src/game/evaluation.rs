use std::convert::Infallible;
use std::str::FromStr;

use super::response::Status;

pub type EvaluationUnit = (char, Status);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Evaluation {
    inner: Vec<EvaluationUnit>,
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
