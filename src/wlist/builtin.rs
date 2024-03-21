use std::fmt::{write, Debug};

use serde_json;

use super::{Word, WordList};

const RAW_WORDLIST_FILE: &str = include_str!("../../data/wordlists/en_US_3b1b_freq_map.json");

#[derive(Clone)]
pub struct BuiltinWList {
    words: super::WordMap,
}

impl super::WordList for BuiltinWList {
    fn solutions(&self) -> Vec<&Word> {
        // PERF: this can be made faster if we were to use parallel iterators or chunking
        // TODO: Filter should be a bit more elegant
        let threshold = self.total_freq() / 2;
        self.wordmap().iter().filter(|i| i.1 > )
    }
    fn length_range(&self) -> impl std::ops::RangeBounds<usize> {
        5..5
    }
    fn wordmap(&self) -> &super::WordMap {
        &self.words
    }
}

impl Default for BuiltinWList {
    fn default() -> Self {
        let words: super::WordMap = serde_json::from_str(RAW_WORDLIST_FILE).unwrap();

        Self { words }
    }
}

impl Debug for BuiltinWList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(
            f,
            format_args!(
                "BuiltinWList {{ amount: {}, total_freq: {} }}",
                self.amount(),
                self.total_freq()
            ),
        )
    }
}
