use libpt::log::{info, trace};

use crate::wlist::word::{ManyWordDatas, Word};
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
    fn guess_for(&self, game: &crate::game::Game<WL>) -> Word {
        // HACK: hardcoded length
        let mut pattern: String = String::from(".....");
        let mut other_chars: Vec<char> = Vec::new();
        let response = game.last_response();
        if response.is_some() {
            for (idx, p) in response
                .unwrap()
                .evaluation()
                .clone()
                .into_iter()
                .enumerate()
            {
                if p.1 == Status::Matched {
                    pattern.replace_range(idx..idx + 1, &p.0.to_string());
                } else if p.1 == Status::Exists {
                    other_chars.push(p.0)
                }
            }
        }
        trace!("other chars: {:?}", other_chars);
        let matches: ManyWordDatas = game
            .wordlist()
            .get_words_matching(pattern)
            .expect("the solution does not exist in the wordlist")
            .iter()
            // only words that have not been guessed yet
            .filter(|p| !game.made_guesses().contains(&&p.0))
            // only words that contain the letters we found earlier (that were not matched)
            .filter(|p| {
                // TODO: don't repeat unmatched contained chars on the same position twice #2
                let mut fits = true;
                for c in other_chars.iter() {
                    fits &= p.0.contains(*c);
                }
                fits
            })
            .map(|v| v.to_owned())
            .collect();
        matches[0].0.to_owned() // FIXME: panicks in interactive solve, when I insert bullshit and
                                // pretend that there is a solution #5 It also crashes in
                                // non-interactive mode
    }
}

impl<'wl, WL: WordList> From<NaiveSolver<'wl, WL>> for AnyBuiltinSolver<'wl, WL> {
    fn from(value: NaiveSolver<'wl, WL>) -> Self {
        Self::Naive(value)
    }
}
