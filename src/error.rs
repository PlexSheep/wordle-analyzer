use thiserror::Error;

use crate::bench::report::Report;
use crate::wlist::word::Word;

pub type WResult<T> = std::result::Result<T, Error>;
pub type GameResult<T> = std::result::Result<T, GameError>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Game Error")]
    GameError {
        #[from]
        source: GameError,
    },
    #[error("Solver Error")]
    SolverError {
        #[from]
        source: SolverError,
    },
    #[error("Benchmark Error")]
    BenchError {
        #[from]
        source: BenchError,
    },
    #[error(transparent)]
    Other {
        #[from]
        source: anyhow::Error,
    },
    // for `FromStr` of `BuiltinSolver`
    #[error("Unknown builtin solver")]
    UnknownBuiltinSolver,
    #[error("pattern matching error")]
    Regex {
        #[from]
        source: regex::Error,
    },
    #[error("Error sharing the benchmark data over multiple threads")]
    Mutex {
        #[from]
        source: std::sync::PoisonError<Report>,
    },
}

#[derive(Debug, Clone, Error)]
pub enum GameError {
    #[error("The guess has the wrong length ({0})")]
    GuessHasWrongLength(usize),
    #[error("The game is finished but a guess is being made")]
    TryingToPlayAFinishedGame,
    #[error("Tried to guess a word that is not in the wordlist ({0})")]
    WordNotInWordlist(Word),
}

#[derive(Debug, Clone, Error)]
pub enum BenchError {
    #[error("Trying to modify a finished report")]
    ModifyFinishedReport,
}

#[derive(Debug, Clone, Error)]
pub enum SolverError {
    #[error("Wordlist has no matches for the gamestate")]
    NoMatches,
}
