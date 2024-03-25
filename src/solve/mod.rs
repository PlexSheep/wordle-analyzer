use std::{fmt::Display, str::FromStr};

use crate::{
    error::{Error, WResult},
    game::{response::*, summary::Summary, Game},
    wlist::{
        word::{Word, WordData},
        WordList,
    },
};

#[cfg(feature = "builtin")]
pub mod naive;
#[cfg(feature = "builtin")]
pub use naive::NaiveSolver;
#[cfg(feature = "builtin")]
pub mod stupid;
#[cfg(feature = "builtin")]
pub use stupid::StupidSolver;

pub trait Solver<'wl, WL: WordList>: Clone + std::fmt::Debug + Sized {
    fn build(wordlist: &'wl WL) -> WResult<Self>;
    fn guess_for(&self, game: &Game<'wl, WL>) -> Word;
    fn make_a_move(&self, game: &mut Game<'wl, WL>) -> WResult<GuessResponse> {
        Ok(game.guess(self.guess_for(game))?)
    }
    fn play(&self, game: &mut Game<'wl, WL>) -> WResult<GuessResponse> {
        let mut resp: GuessResponse;
        loop {
            resp = self.make_a_move(game)?;
            if game.finished() {
                break;
            }
        }
        Ok(resp)
    }
    fn solve(&self, game: &Game<'wl, WL>) -> WResult<Option<WordData>> {
        let mut game = game.clone();
        Ok(self.play(&mut game)?.solution())
    }
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[derive(Debug, Clone)]
pub enum AnyBuiltinSolver<'wl, WL: WordList> {
    Naive(NaiveSolver<'wl, WL>),
    Stupid(StupidSolver<'wl, WL>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BuiltinSolverNames {
    #[default]
    Naive,
    Stupid,
}
impl BuiltinSolverNames {
    pub fn to_solver<'wl, WL: WordList>(&self, wl: &'wl WL) -> AnyBuiltinSolver<'wl, WL> {
        match self {
            Self::Naive => NaiveSolver::build(wl).unwrap().into(),
            Self::Stupid => StupidSolver::build(wl).unwrap().into(),
        }
    }
}

impl FromStr for BuiltinSolverNames {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "naive" => Ok(Self::Naive),
            "stupid" => Ok(Self::Stupid),
            _ => Err(Self::Err::UnknownBuiltinSolver),
        }
    }
}

impl Display for BuiltinSolverNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'wl, WL: WordList> Solver<'wl, WL> for AnyBuiltinSolver<'wl, WL> {
    fn build(wordlist: &'wl WL) -> WResult<Self> {
        Ok(Self::Naive(NaiveSolver::build(wordlist)?))
    }
    fn guess_for(&self, game: &Game<'wl, WL>) -> Word {
        match self {
            Self::Naive(solver) => solver.guess_for(game),
            Self::Stupid(solver) => solver.guess_for(game),
        }
    }
}
