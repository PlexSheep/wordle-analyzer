use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::{Range, RangeBounds};

use crate::error::WResult;

pub(crate) type CharMap = HashMap<char, CharInfo>;

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct CharInfo {
    confirmed_indexes: HashSet<usize>,
    tried_indexes: HashSet<usize>,
    occurences_amount: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SolverState {
    char_map: CharMap,
}

impl SolverState {
    pub fn new() -> Self {
        Self {
            char_map: HashMap::new(),
        }
    }

    pub fn char_map(&self) -> &CharMap {
        &self.char_map
    }

    pub fn char_map_mut(&mut self) -> &mut CharMap {
        &mut self.char_map
    }
}

impl CharInfo {
    pub fn new(word_length: usize) -> Self {
        Self {
            confirmed_indexes: HashSet::new(),
            tried_indexes: HashSet::new(),
            occurences_amount: 0..word_length,
        }
    }

    pub fn found_at(&mut self, idx: usize) {
        self.tried_indexes.insert(idx);
        self.confirmed_indexes.insert(idx);

        if self.occurences_amount.start < 1 {
            self.occurences_amount.start = 1;
        }
    }

    /// tried to guess a char we know exists at this position, but it was incorrect
    pub fn tried_but_failed(&mut self, idx: usize) {
        self.tried_indexes.insert(idx);
    }

    /// char is not in the solution
    pub fn not_in_solution(&mut self) {
        self.occurences_amount.start = 0;
        self.occurences_amount.end = 0;
    }

    pub fn has_been_tried(&self, idx: usize) -> bool {
        self.tried_indexes.contains(&idx)
    }

    #[must_use]
    pub fn part_of_solution(&self) -> bool {
        self.occurences_amount.end > 0
    }

    pub fn at_least_n_occurences(&mut self, n: usize) {
        self.occurences_amount.start = n;
    }

    pub(crate) fn confirmed_indexes(&self) -> &HashSet<usize> {
        &self.confirmed_indexes
    }

    pub(crate) fn confirmed_indexes_mut(&mut self) -> &mut HashSet<usize> {
        &mut self.confirmed_indexes
    }

    pub(crate) fn tried_indexes(&self) -> &HashSet<usize> {
        &self.tried_indexes
    }

    pub(crate) fn tried_indexes_mut(&mut self) -> &mut HashSet<usize> {
        &mut self.tried_indexes
    }

    pub(crate) fn occurences_amount(&self) -> &Range<usize> {
        &self.occurences_amount
    }

    pub(crate) fn occurences_amount_mut(&mut self) -> &mut Range<usize> {
        &mut self.occurences_amount
    }

    pub(crate) fn occurences_of_char_possible(
        &self,
        solution_candidate: &str,
        character: char,
    ) -> bool {
        self.occurences_amount.contains(
            &solution_candidate
                .chars()
                .filter(|c| *c == character)
                .count(),
        )
    }
}

impl Debug for CharInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.part_of_solution() {
            f.debug_struct("CharInfo")
                .field("indexes", &self.confirmed_indexes)
                .field("amnt_occ", &self.occurences_amount)
                .field("tried", &self.tried_indexes)
                .finish()
        } else {
            write!(f, "(not in solution)")
        }
    }
}
