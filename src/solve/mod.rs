use std::{fmt::Display, str::FromStr};

use crate::{
    error::{Error, SolverError, WResult},
    game::{response::*, Game},
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

/// Trait for any datatype that can solve [Games][Game].
///
/// # Examples
///
/// For a detailed example, see the
/// [`wordlesolve`](https://git.cscherr.de/PlexSheep/wordle-analyzer/src/branch/master/src/bin/solve/simple.rs)
/// binary.
///
/// # Builtins
///
/// This [crate] implements a few builtin [Solvers][Solver]:
///
/// * [Naive](NaiveSolver) - Keep the found letters and use letters that are confirmed to be
///                          contained. This is probably the closest thing to how a human would
///                          play wordle.
/// * [Stupid](StupidSolver) - Guesses random words that have not been guessed yet.
///
/// If you want to have the user select a model, create an enum with it's variants containing your
/// [Solvers][Solver] and have this enum implement [Solver], see [AnyBuiltinSolver].
pub trait Solver<'wl, WL: WordList>: Clone + std::fmt::Debug + Sized + Sync {
    /// Build and initialize a [Solver]
    fn build(wordlist: &'wl WL) -> WResult<Self>;
    /// Calculate the next guess for a [Game]
    ///
    /// Each [Solver] needs to implement this method themselves, many other methods rely on this to
    /// play the [Game], such as [play](Solver::play) or [solve](Solver::solve).
    fn guess_for(&self, game: &Game<'wl, WL>) -> WResult<Word>;
    /// Make a singular step for a [Game]
    ///
    /// # Errors
    ///
    /// This function will return an error if [guess_for](Solver::guess_for) fails.
    fn make_a_move(&self, game: &mut Game<'wl, WL>) -> WResult<GuessResponse> {
        Ok(game.guess(self.guess_for(game)?, None)?)
    }
    /// Play a [Game] and return the last [GuessResponse].
    ///
    /// # Errors
    ///
    /// This function will return an error if [make_a_move](Solver::make_a_move) fails.
    fn play(&self, game: &mut Game<'wl, WL>) -> WResult<GuessResponse> {
        // TODO: check if the game is finished already and return an Err if so
        let mut resp: GuessResponse;
        loop {
            resp = self.make_a_move(game)?;
            if game.finished() {
                break;
            }
        }
        Ok(resp)
    }
    /// Play a [Game] and return the last [GuessResponse].
    ///
    /// Like [play](Solver::play) but takes an owned game instead of a mutable reference.
    ///
    /// # Errors
    ///
    /// This function will return an error if [make_a_move](Solver::make_a_move) fails.
    fn play_owned(&self, mut game: Game<'wl, WL>) -> WResult<GuessResponse> {
        let mut resp: GuessResponse;
        loop {
            resp = self.make_a_move(&mut game)?;
            if game.finished() {
                break;
            }
        }
        Ok(resp)
    }
    /// Play a [Game] and return the solution if one was found
    ///
    /// If no solution was found, this function will return [None].
    ///
    /// # Errors
    ///
    /// This function will return an error if [play](Solver::play) fails.
    fn solve(&self, game: &Game<'wl, WL>) -> WResult<Option<WordData>> {
        let mut game = game.clone();
        Ok(self.play(&mut game)?.solution())
    }
    /// Box the [Solver]
    ///
    /// Returns a [Box] containing the [Solver].
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
    type Err = SolverError;
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
    fn guess_for(&self, game: &Game<'wl, WL>) -> WResult<Word> {
        Ok(match self {
            Self::Naive(solver) => solver.guess_for(game)?,
            Self::Stupid(solver) => solver.guess_for(game)?,
        })
    }
}
