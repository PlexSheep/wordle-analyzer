use std::fmt::{Debug, Display};
use std::path::Path;

use serde_json;

use crate::error::WordlistError;

use super::{Word, WordList};

pub const RAW_WORDLIST_BUNDLED_ENGLISH: &str =
    include_str!("../../data/wordlists/en_US_3b1b_freq_map.json");
pub const RAW_WORDLIST_BUNDLED_GERMAN_SMALL: &str =
    include_str!("../../data/wordlists/german_SUBTLEX-DE_small.json");
pub const RAW_WORDLIST_PATH_ENGLISH: &str = "../../data/wordlists/en_US_3b1b_freq_map.json";
pub const RAW_WORDLIST_PATH_GERMAN_FULL: &str = "../../data/wordlists/german_SUBTLEX-DE_full.json";
pub const RAW_WORDLIST_PATH_GERMAN_SMALL: &str = "../../data/wordlists/german_SUBTLEX-DE_full.json";

#[derive(Clone)]
pub struct BuiltinWList {
    words: super::WordMap,
    name: String,
}

impl BuiltinWList {
    /// load a wordlist from file
    ///
    /// Wordlist files are expected to have the following format:
    ///
    /// ```json
    /// {
    ///     "word": 0.001
    /// }
    /// ```
    ///
    /// Where the number is the frequency. Higher/Lower case is ignored.
    ///
    /// Only words with the specified length will be included.
    ///
    /// ## Errors
    ///
    /// Will fail if the file path cannot be read or the format is wrong.
    pub fn load<P: AsRef<std::path::Path>>(wl_path: P, len: usize) -> Result<Self, WordlistError> {
        let path: &Path = wl_path.as_ref();
        let file = std::fs::File::open(path)?;

        // don't load the whole string into memory
        let reader = std::io::BufReader::new(file);
        let mut words: super::WordMap = serde_json::from_reader(reader)?;
        words.only_words_with_len(len);

        let name: String = if let Some(osstr) = path.file_name() {
            osstr.to_str().unwrap_or("(no name)").to_string()
        } else {
            "(no name)".to_string()
        };

        Ok(Self { words, name })
    }

    pub fn english(len: usize) -> Self {
        let mut words: super::WordMap = serde_json::from_str(RAW_WORDLIST_BUNDLED_ENGLISH).unwrap();
        words.only_words_with_len(len);

        Self {
            words,
            name: "(builtin english)".to_string(),
        }
    }

    pub fn german(len: usize) -> Self {
        let mut words: super::WordMap =
            serde_json::from_str(RAW_WORDLIST_BUNDLED_GERMAN_SMALL).unwrap();
        words.only_words_with_len(len);

        Self {
            words,
            name: "(builtin german)".to_string(),
        }
    }
}

impl super::WordList for BuiltinWList {
    fn length_range(&self) -> impl std::ops::RangeBounds<usize> {
        5..5
    }
    fn wordmap(&self) -> &super::WordMap {
        &self.words
    }
    fn get_word(&self, word: &Word) -> Option<super::WordData> {
        self.words.get(word)
    }
}

impl Default for BuiltinWList {
    fn default() -> Self {
        Self::english(5)
    }
}

impl Debug for BuiltinWList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BuiltinWList")
            .field("name", &self.name)
            .field("words", &self.words)
            .finish()
    }
}

impl Display for BuiltinWList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}:\nwords:\t{}\ntop 5:\t{:?}",
            self.name,
            self.len(),
            self.n_most_likely(5)
        )
    }
}
