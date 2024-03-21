use serde_json;

use super::Word;

const RAW_WORDLIST_FILE: &str = include_str!("../../data/wordlists/en_US_3b1b_freq_map.json");

#[derive(Clone, Debug)]
pub struct BuiltinWList {
    words: super::WordMap,
}

impl super::WordList for BuiltinWList {
    fn solutions(&self) -> Vec<&Word> {
        // PERF: this can be made faster if we were to use parallel iterators or chunking
        self.words.keys().collect()
    }
    fn length_range(&self) -> impl std::ops::RangeBounds<usize> {
        5..5
    }
}

impl Default for BuiltinWList {
    fn default() -> Self {
        let words: super::WordMap = serde_json::from_str(RAW_WORDLIST_FILE).unwrap();

        Self { words }
    }
}
