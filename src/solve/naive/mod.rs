use std::collections::HashMap;

use libpt::log::{debug, info, trace};

use crate::error::{SolverError, WResult};
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
        // char and then the indexes we tried for that already
        let mut other_chars: HashMap<char, Vec<usize>> = HashMap::new();
        // a hash map telling how many of the characters may be in a correct word (+1)
        // if the value for a char is 2 => it may be in the solution 1 time
        // if the value for a char is 1 => it may not be in the solution
        let mut wrong_chars: Vec<char> = Vec::new();
        let responses = game.responses().iter().enumerate();
        for (_idx, response) in responses {
            for (idx, p) in response.evaluation().clone().into_iter().enumerate() {
                match p.1 {
                    Status::Matched => {
                        pattern.replace_range(idx..idx + 1, &p.0.to_string());
                    }
                    Status::Exists => {
                        other_chars.entry(p.0).or_default();
                        let v = other_chars.get_mut(&p.0).unwrap();
                        v.push(idx);
                    }
                    Status::None => wrong_chars.push(p.0),
                }
            }
        }
        debug!("other chars: {:?}", other_chars);
        debug!("wrong chars: {:?}", wrong_chars);

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
                        for spot in other.1 {
                            already_tried.push((*spot, *other.0));
                        }

                        if p.0
                            .char_indices()
                            .filter(|ci| ci.1 == *other.0 && !already_tried.contains(ci))
                            .count()
                            < 1
                        {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            })
            // only words that do not contain the letters we know are wrong
            .filter(|p| {
                for wrong in &wrong_chars {
                    let in_other = other_chars.iter().filter(|v| *v.0 == *wrong).count()
                        + pattern.chars().filter(|v| *v == *wrong).count();
                    if p.0.contains(*wrong) {
                        // HACK: something is still not quite right here, we basically ignore all
                        // wrong chars if they occur in other_chars at least once, which can't be
                        // fully correct.
                        if in_other > 0 {
                            continue;
                        }
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
