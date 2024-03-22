use thiserror::Error;

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
}

#[derive(Debug, Clone, Error)]
pub enum GameError {
    #[error("The guess has the wrong length")]
    GuessHasWrongLength,
    #[error("The game is finished but a guess is being made")]
    TryingToPlayAFinishedGame,
}
