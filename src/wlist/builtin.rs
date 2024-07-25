use std::fmt::{write, Debug, Display};
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
    /// ## Errors
    ///
    /// Will fail if the file path cannot be read or the format is wrong.
    pub fn load<P: AsRef<std::path::Path>>(wl_path: P) -> Result<Self, WordlistError> {
        let path: &Path = wl_path.as_ref();
        let file = std::fs::File::open(path)?;

        // don't load the whole string into memory
        let reader = std::io::BufReader::new(file);
        let words: super::WordMap = serde_json::from_reader(reader)?;

        Ok(Self { words })
    }

    pub fn english() -> Self {
        let words: super::WordMap = serde_json::from_str(RAW_WORDLIST_BUNDLED_ENGLISH).unwrap();

        Self { words }
    }

    pub fn german() -> Self {
        let words: super::WordMap =
            serde_json::from_str(RAW_WORDLIST_BUNDLED_GERMAN_SMALL).unwrap();

        Self { words }
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
        Self::english()
    }
}

impl Debug for BuiltinWList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(
            f,
            format_args!(
                "BuiltinWList {{ \n\
                \tamount: {}, \n\
                \ttotal_freq: {}, \n\
                \tcommon: {}, \n\
                \tthreshold: {}, \n\
                \tfreq_range: {:?}, \n\
                \tover_threshold: {:#?}, \n\
                }}",
                self.amount(),
                self.total_freq(),
                self.wordmap().n_common(),
                self.wordmap().threshold(),
                self.wordmap().freq_range(),
                self.over_threashold()
            ),
        )
    }
}

impl Display for BuiltinWList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}
