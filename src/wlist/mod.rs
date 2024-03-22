use rand::seq::IteratorRandom;

use std::collections::HashMap;
use std::ops::RangeBounds;

#[cfg(feature = "builtin_wlist")]
pub mod builtin;
pub mod word;
use word::*;

use crate::error::WResult;

pub type AnyWordlist = Box<dyn WordList>;

pub trait WordList: Clone + std::fmt::Debug + Default {
    fn solutions(&self) -> ManyWordDatas {
        let wmap = self.wordmap();
        let threshold = wmap.threshold();
        wmap.iter().filter(|i| *i.1 > threshold).collect()
    }
    fn rand_solution(&self) -> WordData {
        let mut rng = rand::thread_rng();
        let sol = *self.solutions().iter().choose(&mut rng).unwrap();
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
    fn letter_frequency(&self) -> WordMap {
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
        // make all chars to strings
        let cmap: HashMap<Word, usize> = cmap.into_iter().map(|p| (p.0.to_string(), p.1)).collect();
        WordMap::from_absolute(cmap)
    }
}
