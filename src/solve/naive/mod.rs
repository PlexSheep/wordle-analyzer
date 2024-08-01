use std::collections::HashMap;

use libpt::log::{debug, info, trace};

use crate::error::{SolverError, WResult};
use crate::game::evaluation::Evaluation;
use crate::wlist::word::{Word, WordData};
use crate::wlist::WordList;

use super::{AnyBuiltinSolver, Solver, Status};

#[derive(Debug, Clone)]
pub struct NaiveSolver<'wl, WL> {
    wl: &'wl WL,
}

impl<'wl, WL: WordList> Solver<'wl, WL> for NaiveSolver<'wl, WL> {
    fn build(wordlist: &'wl WL) -> crate::error::WResult<Self> {
        info!("using naive solver");
        Ok(Self { wl: wordlist })
    }
    /// Guess a word from the wordlist for the given game
    ///
    /// ## Algorithm
    ///
    /// * Look at the evaluation for the last response and keep the correct letters
    /// * Get all words that have these letters at the right position
    /// * Discard words that have already been tried
    /// * Discard all words that don't have the chars that we know from the last guess are in the
    ///   word, but don't know the position of.
    fn guess_for(&self, game: &crate::game::Game<WL>) -> WResult<Word> {
        let mut pattern: String = ".".repeat(game.length());
        // indexes we tried for that char and the number of occurences
        let mut other_chars: HashMap<char, (Vec<usize>, usize)> = HashMap::new();
        let responses = game.responses().iter().enumerate();
        for (_idx, response) in responses {
            let evaluation: &Evaluation = response.evaluation();
            for (idx, p) in evaluation.clone().into_iter().enumerate() {
                match p.1 {
                    Status::Matched => {
                        pattern.replace_range(idx..idx + 1, &p.0.to_string());

                        other_chars.entry(p.0).or_default();
                        let v = other_chars.get_mut(&p.0).unwrap();
                        v.1 += 1;
                    }
                    Status::Exists => {
                        other_chars.entry(p.0).or_default();
                        let v = other_chars.get_mut(&p.0).unwrap();
                        v.0.push(idx);

                        // TODO: count how many times the char occurs
                        v.1 += 1;
                    }
                    Status::None => {
                        other_chars.entry(p.0).or_default();
                    }
                }
            }
        }
        debug!("other chars: {:?}", other_chars);

        // get all words that have the correct chars on the same positions
        let mut matches: Vec<WordData> = game.wordlist().get_words_matching(&pattern)?;
        if matches.is_empty() {
            return Err(SolverError::NoMatches(game.solution().cloned()).into());
        }
        matches = matches
            .iter()
            // only words that have not been guessed yet
            .filter(|p| !game.made_guesses().contains(&&p.0))
            // only words that do contain the chars we know exist
            .filter(|p| {
                for other in other_chars.iter() {
                    if p.0.contains(*other.0) {
                        let mut already_tried: Vec<(_, _)> = Vec::new();
                        for spot in &other.1 .0 {
                            already_tried.push((spot, *other.0));
                        }

                        if p.0.chars().filter(|c| *c == *other.0).count() > other.1 .1 {
                            return false; // the char occurs too often in that word
                        }
                        for c in p.0.char_indices() {
                            if c.1 == *other.0 && other.1 .0.contains(&c.0) {
                                return false;
                            }
                        }
                    } else if other.1 .1 != 0 {
                        return false;
                    }
                }
                true
            })
            .map(|v| v.to_owned())
            .collect();
        if matches.is_empty() {
            return Err(SolverError::NoMatches(game.solution().cloned()).into());
        }
        Ok(matches[0].0.to_owned())
    }
}

impl<'wl, WL: WordList> From<NaiveSolver<'wl, WL>> for AnyBuiltinSolver<'wl, WL> {
    fn from(value: NaiveSolver<'wl, WL>) -> Self {
        Self::Naive(value)
    }
}
