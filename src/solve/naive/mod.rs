use std::collections::HashMap;

use libpt::log::{debug, error, info, trace};

use crate::error::{SolverError, WResult};
use crate::game::evaluation::{Evaluation, EvaluationUnit};
use crate::game::response::Status;
use crate::wlist::word::{Word, WordData};
use crate::wlist::WordList;

mod states;
use states::*;

use super::{AnyBuiltinSolver, Solver};

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
        let mut state: SolverState = SolverState::new();
        let responses = game.responses().iter().enumerate();
        for (_idx, response) in responses {
            let mut abs_freq: HashMap<char, usize> = HashMap::new();
            let evaluation: &Evaluation = response.evaluation();
            for (idx, p) in evaluation.clone().into_iter().enumerate() {
                state.start_step();
                match p.1 {
                    Status::Matched => {
                        pattern.replace_range(idx..idx + 1, &p.0.to_string());

                        state
                            .char_map_mut()
                            .entry(p.0)
                            .or_insert(CharInfo::new(game.length()))
                            .found_at(idx);
                        *abs_freq.entry(p.0).or_default() += 1;
                    }
                    Status::Exists => {
                        let cinfo = state
                            .char_map_mut()
                            .entry(p.0)
                            .or_insert(CharInfo::new(game.length()));
                        cinfo.tried_but_failed(idx);
                        *abs_freq.entry(p.0).or_default() += 1;
                    }
                    Status::None => {
                        let cinfo = state
                            .char_map_mut()
                            .entry(p.0)
                            .or_insert(CharInfo::new(game.length()));
                        cinfo.tried_but_failed(idx);
                    }
                }
                trace!("absolute frequencies: {abs_freq:?}");
                state.finish_step(&abs_freq);
            }
        }

        debug!("built state from responses: {state:#?}");

        // get all words that have the correct chars on the same positions
        let mut matches: Vec<WordData> = game.wordlist().get_words_matching(&pattern)?;
        if matches.is_empty() {
            error!("no matches even when just considering the known good chars");
            return Err(SolverError::NoMatches(game.solution().cloned()).into());
        } else {
            trace!("found {} basic matches", matches.len())
        }
        matches = matches
            .iter()
            // only words that have not been guessed yet
            .filter(|p| !game.made_guesses().contains(&&p.0))
            .filter(|solution_candidate| {
                if !game.responses().is_empty()
                    && !state.has_all_known_contained(&solution_candidate.0)
                {
                    trace!("known cont:{:#?}", state.get_all_known_contained());
                    return false;
                }
                for (idx, c) in solution_candidate.0.char_indices() {
                    let cinfo = state
                        .char_map_mut()
                        .entry(c)
                        .or_insert(CharInfo::new(game.length()));
                    if !cinfo.occurences_of_char_possible(&solution_candidate.0, c)
                        || cinfo.has_been_tried(idx)
                    {
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
