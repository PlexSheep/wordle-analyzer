use std::collections::HashMap;
use std::fmt::write;

use libpt::log::trace;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type Frequency = f64;

// PERF: Hash for String is probably a bottleneck
pub type Word = String;
pub type WordData = (Word, Frequency);
pub type WordDataRef<'wl> = (&'wl Word, &'wl Frequency);
pub type ManyWordsRef<'a> = Vec<&'a Word>;
pub type ManyWordDatas = Vec<(Word, Frequency)>;

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WordMap {
    #[serde(flatten)]
    inner: HashMap<Word, Frequency>,
}

impl Default for WordMap {
    fn default() -> Self {
        Self::new()
    }
}

impl WordMap {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, String, Frequency> {
        self.inner.keys()
    }
    pub fn values(&self) -> std::collections::hash_map::Values<'_, String, Frequency> {
        self.inner.values()
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, Frequency> {
        self.inner.iter()
    }
    pub fn freq_range(&self) -> std::ops::Range<Frequency> {
        // TODO: calculate this instead of estimating like this
        return 0.1e-10..1e-6;
        let lowest: Frequency = todo!();
        let highest: Frequency = todo!();
        lowest..highest
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn n_common(&self) -> usize {
        // TODO: calculate the amount of relatively common words
        3000
    }
    pub fn threshold(&self) -> Frequency {
        // HACK: I completely butchered the math here
        // see https://github.com/3b1b/videos/blob/master/_2022/wordle/simulations.py
        let l_under_sigmoid = 10_f64;
        let len = self.len();
        let mut c: f64 = l_under_sigmoid * (0.5 + self.n_common() as f64 / len as f64);
        c *= 1e-7;
        trace!(threshold = c);
        c
    }
    pub fn inner(&self) -> &HashMap<Word, Frequency> {
        &self.inner
    }
    pub fn get<I: std::fmt::Display>(&self, word: I) -> Option<WordData> {
        self.inner
            .get(&word.to_string())
            .map(|f| (word.to_string(), *f))
    }
    pub fn from_absolute(abs: HashMap<Word, usize>) -> Self {
        let n: f64 = abs.keys().len() as f64;
        let relative: HashMap<Word, Frequency> =
            abs.into_iter().map(|p| (p.0, p.1 as f64 / n)).collect();
        relative.into()
    }
}

impl std::fmt::Debug for WordMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(
            f,
            format_args!(
                "WordMap {{\n\
                    \t\tlen: {}\n\
                    \t\tfreq_range: {:?}\n\
                    \t\tcommon: {:?}\n\
                \t}}",
                self.len(),
                self.freq_range(),
                self.n_common()
            ),
        )
    }
}

impl From<HashMap<Word, Frequency>> for WordMap {
    fn from(value: HashMap<Word, Frequency>) -> Self {
        Self { inner: value }
    }
}

impl From<WordMap> for HashMap<Word, Frequency> {
    fn from(value: WordMap) -> Self {
        value.inner
    }
}
