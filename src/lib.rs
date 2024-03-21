#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

/// Default letters of a solution word
pub const DEFAULT_WORD_LENGTH: usize = 5;
/// Default amount of guesses per game
pub const DEFAULT_MAX_STEPS: usize = 6;

pub mod error;
#[cfg(feature = "bench")]
pub mod bench;
#[cfg(feature = "game")]
pub mod game;
#[cfg(feature = "solvers")]
pub mod solvers;
pub mod wlist;
