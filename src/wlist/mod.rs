use rand::{prelude::*, seq::IteratorRandom};
use std::collections::HashMap;
use std::ops::RangeBounds;

#[cfg(feature = "builtin_wlist")]
pub mod builtin;
pub mod word;
use word::*;

pub type AnyWordlist = Box<dyn WordList>;

pub trait WordList: Clone + std::fmt::Debug + Default {
    fn solutions(&self) -> ManySolutions {
        let wmap = self.wordmap();
        let threshold = wmap.threshold();
        wmap.iter().filter(|i| *i.1 > threshold).collect()
    }
    fn rand_solution(&self) -> Solution {
        let mut rng = rand::thread_rng();
        let sol = *self.solutions().iter().choose(&mut rng).unwrap();
        (sol.0.to_owned(), sol.1.to_owned())
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
        WordMap::new(hm)
    }
}
