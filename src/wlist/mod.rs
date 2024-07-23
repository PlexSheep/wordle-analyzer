use rand::seq::IteratorRandom;

use regex::Regex;

use std::collections::HashMap;
use std::fmt::Display;
use std::ops::RangeBounds;

#[cfg(feature = "builtin")]
pub mod builtin;
pub mod word;
use word::*;

use crate::error::WResult;

pub type AnyWordlist = Box<dyn WordList>;

pub trait WordList: Clone + std::fmt::Debug + Default + Sync + Display {
    fn solutions(&self) -> Vec<WordData> {
        let wmap = self.wordmap().clone();
        let threshold = wmap.threshold();
        wmap.iter()
            .filter(|i| *i.1 > threshold)
            .map(|p| (p.0.clone(), *p.1))
            .collect()
    }
    fn rand_solution(&self) -> WordData {
        let mut rng = rand::thread_rng();
        let sol = self.solutions().iter().choose(&mut rng).unwrap().clone();
        (sol.0.to_owned(), sol.1.to_owned())
    }
    fn rand_word(&self) -> WordData {
        let mut rng = rand::thread_rng();
        let w = self.wordmap().iter().choose(&mut rng).unwrap();
        (w.0.clone(), *w.1)
    }
    fn length_range(&self) -> impl RangeBounds<usize>;
    fn amount(&self) -> usize {
        self.solutions().len()
    }
    fn wordmap(&self) -> &WordMap;
    fn total_freq(&self) -> Frequency {
        self.wordmap().values().map(|a| a.to_owned()).sum()
    }
    fn sort_likelihood(&self) -> Vec<WordData> {
        let wmap = self.wordmap();
        let mut wpairs: Vec<(_, _)> = wmap.iter().collect();
        wpairs.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap().reverse());
        wpairs
            .iter()
            .map(|v| (v.0.to_owned(), v.1.to_owned()))
            .collect()
    }
    fn n_most_likely(&self, n: usize) -> Vec<WordData> {
        self.sort_likelihood().into_iter().take(n).collect()
    }
    fn over_threashold(&self) -> WordMap {
        let wmap = self.wordmap();
        let threshold = wmap.threshold();
        let wpairs: Vec<(_, _)> = wmap.iter().filter(|i| *i.1 > threshold).collect();
        let mut hm = HashMap::new();
        for (k, v) in wpairs {
            hm.insert(k.into(), *v);
        }
        WordMap::from(hm)
    }
    fn get_word(&self, word: &Word) -> Option<WordData>;
    fn letter_frequency(&self) -> HashMap<char, Frequency> {
        // PERF: this function has complexity O(n²)!
        let mut cmap: HashMap<char, usize> = HashMap::new();
        // count the chars in each word
        for word in self.wordmap().iter() {
            for c in word.0.chars() {
                if let Some(inner_value) = cmap.get_mut(&c) {
                    *inner_value += 1;
                }
            }
        }
        // convert to relative frequency
        let n: f64 = cmap.keys().len() as f64;
        cmap.into_iter().map(|p| (p.0, p.1 as f64 / n)).collect()
    }
    fn raw_wordlist(&self) -> String {
        let mut buf = String::new();
        for w in self.wordmap().keys() {
            buf += w;
            buf += "\n";
        }
        buf
    }
    fn get_words_matching(&self, pattern: String) -> WResult<Vec<WordData>> {
        let pattern = Regex::new(&pattern)?;
        let hay = self.raw_wordlist();
        let keys = pattern.captures_iter(&hay);
        let mut buf = Vec::new();
        for k in keys {
            let w: WordData = self.wordmap().get(&k[0]).unwrap();
            buf.push(w)
        }
        // sort by frequency
        buf.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        buf.reverse();
        Ok(buf)
    }
}
