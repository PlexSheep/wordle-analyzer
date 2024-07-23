use core::panic;
use std::fmt::Display;

use crate::error::*;
use crate::wlist::word::{ManyWordsRef, Word, WordData};
use crate::wlist::WordList;

use libpt::log::{debug, trace};

pub mod response;
use response::GuessResponse;

pub mod evaluation;

pub mod summary;

use self::evaluation::Evaluation;
use self::response::Status;

#[derive(Debug, Clone, PartialEq)]
pub struct Game<'wl, WL>
where
    WL: WordList,
{
    length: usize,
    precompute: bool,
    max_steps: usize,
    step: usize,
    solution: Option<WordData>,
    wordlist: &'wl WL,
    responses: Vec<GuessResponse>,
    // TODO: keep track of the letters the user has tried
}

impl<'wl, WL: WordList> Game<'wl, WL> {
    /// get a new [`GameBuilder`]
    pub fn builder(wl: &'wl WL) -> GameBuilder<'wl, WL> {
        GameBuilder::new(wl, true)
    }
    /// Create a [Game] of wordle
    ///
    /// This method will load the wordlist, determine if a word may be used as a solution for a
    /// game, and select a solution at random.
    ///
    /// It will also set a few values to their initial state.
    ///
    /// Don't use this method directly, instead, make use of the [`GameBuilder`].
    ///
    /// # Errors
    ///
    /// No Errors
    pub fn build(
        length: usize,
        precompute: bool,
        max_steps: usize,
        wlist: &'wl WL,
        generate_solution: bool,
    ) -> GameResult<Self> {
        // TODO: check if the length is in the range bounds of the wordlist
        let game: Game<'wl, WL> = Game {
            length,
            precompute,
            max_steps,
            step: 0,
            solution: if generate_solution {
                Some(wlist.rand_solution())
            } else {
                None
            },
            wordlist: wlist,
            responses: Vec::new(),
        };

        Ok(game)
    }

    /// set a solution, can be used for testing
    pub fn set_solution(&mut self, sol: Option<WordData>) {
        self.solution = sol;
    }

    /// Make a new guess
    ///
    /// The word will be evaluated against the [solution](Game::solution) of the [Game].
    /// A [GuessResponse] will be formulated, showing us which letters are correctly placed, in the
    /// solution, or just wrong.
    ///
    /// Note that you do not need to use the [GuessResponse], it is appended to the game state.
    ///
    /// # Errors
    ///
    /// This function will return an error if the length of the [Word] is wrong It will also error
    /// if the game is finished.
    pub fn guess(&mut self, guess: Word, eval: Option<Evaluation>) -> GameResult<GuessResponse> {
        if guess.len() != self.length {
            return Err(GameError::GuessHasWrongLength(guess.len()));
        }
        if self.finished() || self.step > self.max_steps {
            return Err(GameError::TryingToPlayAFinishedGame);
        }
        if self.wordlist.get_word(&guess).is_none() {
            return Err(GameError::WordNotInWordlist(guess));
        }
        self.step += 1;

        let response;
        if eval.is_some() && self.solution.is_none() {
            response = GuessResponse::new(&guess, eval.unwrap(), self);
        } else if let Some(solution) = self.solution.clone() {
            response = GuessResponse::new(&guess, Self::evaluate(solution, &guess), self);
        } else {
            panic!("there is neither an evaluation nor a predefined solution for this guess");
        }
        self.responses.push(response.clone());
        Ok(response)
    }

    pub fn evaluate(mut solution: WordData, guess: &Word) -> Evaluation {
        let mut evaluation = Vec::new();
        let mut status: Status;
        for (idx, c) in guess.chars().enumerate() {
            if solution.0.chars().nth(idx) == Some(c) {
                status = Status::Matched;
                solution.0.replace_range(idx..idx + 1, "_");
            } else if solution.0.contains(c) {
                status = Status::Exists;
                solution.0 = solution.0.replacen(c, "_", 1);
            } else {
                status = Status::None
            }
            evaluation.push((c, status));
        }
        evaluation.into()
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn solution(&self) -> Option<&WordData> {
        self.solution.as_ref()
    }

    pub fn step(&self) -> usize {
        self.step
    }

    pub fn finished(&self) -> bool {
        if self.responses().is_empty() {
            return false;
        }
        self.responses().last().unwrap().finished()
    }

    pub fn won(&self) -> bool {
        if self.responses().is_empty() {
            return false;
        }
        self.responses().last().unwrap().won()
    }

    pub fn max_steps(&self) -> usize {
        self.max_steps
    }
    pub fn responses(&self) -> &Vec<GuessResponse> {
        &self.responses
    }
    pub fn last_response(&self) -> Option<&GuessResponse> {
        self.responses().last()
    }

    pub fn wordlist(&self) -> &WL {
        self.wordlist
    }

    pub(crate) fn made_guesses(&self) -> ManyWordsRef {
        self.responses.iter().map(|r| r.guess()).collect()
    }
}

/// Build and Configure a [`Game`]
///
/// This struct is used to build and configure a [`Game`] of Wordle.
///
/// ## Examples
///
/// [`GameBuilder`] implements [`Default`]. [`Game::builder`] uses [`GameBuilder::default`].
/// You don't need to set custom values if you accept the defaults.
///
/// ```
/// # use wordle_analyzer::game::*;
/// # use wordle_analyzer::wlist::builtin::BuiltinWList;
/// # use anyhow::Result;
/// # fn main() -> Result<()> {
/// let wl = BuiltinWList::default();
/// let game: Game<_> = GameBuilder::new(&wl, true)
///     .build()?;
/// # Ok(())
/// # }
/// ```
///
/// ```
/// # use wordle_analyzer::game::*;
/// # use wordle_analyzer::wlist::builtin::BuiltinWList;
/// # use anyhow::Result;
/// # fn main() -> Result<()> {
/// let wl = BuiltinWList::default();
/// let game: Game<_> = Game::builder(&wl)
///     .length(5)
///     .precompute(false)
///     .max_steps(6)
///     .build()?;
/// # Ok(())
/// # }
/// ```
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameBuilder<'wl, WL: WordList> {
    length: usize,
    precompute: bool,
    max_steps: usize,
    wordlist: &'wl WL,
    generate_solution: bool,
}

impl<'wl, WL: WordList> GameBuilder<'wl, WL> {
    /// make a new [GameBuilder]
    ///
    /// We need a [WordList], so provide one here.
    pub fn new(wl: &'wl WL, generate_solution: bool) -> Self {
        Self {
            length: super::DEFAULT_WORD_LENGTH,
            precompute: false,
            max_steps: super::DEFAULT_MAX_STEPS,
            wordlist: wl,
            generate_solution,
        }
    }

    /// build a [`Game`] with the stored configuration
    pub fn build(&'wl self) -> GameResult<Game<'wl, WL>> {
        trace!("{:#?}", self);
        let game: Game<WL> = Game::build(
            self.length,
            self.precompute,
            self.max_steps,
            self.wordlist,
            self.generate_solution,
        )?;
        Ok(game)
    }

    /// Should we pre compute all possible answers? This will make startup significantly more
    /// expensive, but reduce the computing time while playing.
    ///
    /// Default is [`false`]
    pub fn precompute(mut self, precompute: bool) -> Self {
        self.precompute = precompute;
        self
    }

    /// Set the length of words for the game
    ///
    /// Default is [`super::DEFAULT_WORD_LENGTH`]
    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    /// Set the amount of guesses per game
    ///
    /// Default is [`super::DEFAULT_MAX_STEPS`]
    pub fn max_steps(mut self, max_steps: usize) -> Self {
        self.max_steps = max_steps;
        debug!("max steps: {:#?}", self.max_steps);
        self
    }

    /// Set the wordlist for the builder
    ///
    /// The builder can be used multiple times. Each [`Game`] will have a immutable reference to
    /// `wl`.
    pub fn wordlist(mut self, wl: &'wl WL) -> Self {
        self.wordlist = wl;
        self
    }
}

impl<'wl, WL: WordList> Display for Game<'wl, WL> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: make this actually useful
        // TODO: make this actually fancy
        write!(
            f,
            "turn:\t\t{}\nsolution:\t{:?}\nguesses:\t",
            self.step(),
            self.solution(),
        )?;
        for s in self
            .responses()
            .iter()
            .map(|v| v.evaluation().to_owned().colorized_display(v.guess()))
        {
            write!(f, "\"")?;
            for si in s {
                write!(f, "{si}")?;
            }
            write!(f, "\", ")?;
        }
        Ok(())
    }
}
