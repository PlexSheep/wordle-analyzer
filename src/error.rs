use thiserror::Error;

use crate::wlist::word::Word;

pub type WResult<T> = std::result::Result<T, Error>;
pub type GameResult<T> = std::result::Result<T, GameError>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("GameError")]
    GameError {
        #[from]
        source: GameError,
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
