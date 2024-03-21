use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// NOTE: We might need a different implementation for more precision
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Frequency {
    inner: f64,
}
// PERF: Hash for String is probably a bottleneck
pub type Word = String;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WordMap {
    inner: HashMap<Word, Frequency>,
}

impl WordMap {
    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, String, Frequency> {
        self.inner.keys()
    }
}
