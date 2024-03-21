use rand::{prelude::*, seq::IteratorRandom};
use std::collections::HashMap;
use std::ops::RangeBounds;

#[cfg(feature = "builtin_wlist")]
pub mod builtin;
pub mod word;
use word::*;

pub type AnyWordlist = Box<dyn WordList>;

pub trait WordList: Clone + std::fmt::Debug + Default {
    // NOTE: The possible answers should be determined with a wordlist that has the
    // frequencies/probabilities of the words. We then use a sigmoid function to determine if a
    // word can be a solution based on that value. Only words above some threshold of
    // commonness will be available as solutions then. Next, we choose one of the allowed words
    // randomly.
    // NOTE: must never return nothing
    fn solutions(&self) -> Vec<&Word>;
    fn rand_solution(&self) -> &Word {
        let mut rng = rand::thread_rng();
        self.solutions().iter().choose(&mut rng).unwrap()
    }
    fn length_range(&self) -> impl RangeBounds<usize>;
}
