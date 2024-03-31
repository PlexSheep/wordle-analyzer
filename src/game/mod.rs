use crate::error::*;
use crate::wlist::word::{ManyWordDatas, ManyWordsRef, Word, WordData};
use crate::wlist::WordList;

use libpt::log::debug;

pub mod response;
use response::GuessResponse;

pub mod summary;

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
    solution: WordData,
    wordlist: &'wl WL,
    finished: bool,
    responses: Vec<GuessResponse>,
    // TODO: keep track of the letters the user has tried
}

impl<'wl, WL: WordList> Game<'wl, WL> {
    /// get a new [`GameBuilder`]
    pub fn builder(wl: &'wl WL) -> GameBuilder<'wl, WL> {
        GameBuilder::new(wl)
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
    /// This function will return an error if .
    pub(crate) fn build(
        length: usize,
        precompute: bool,
        max_steps: usize,
        wlist: &'wl WL,
    ) -> GameResult<Self> {
        // TODO: check if the length is in the range bounds of the wordlist
        let solution = wlist.rand_solution();
        let game: Game<'wl, WL> = Game {
            length,
            precompute,
            max_steps,
            step: 1,
            solution,
            wordlist: wlist,
            finished: false,
            responses: Vec::new(),
        };

        Ok(game)
    }

    /// Make a new guess
    ///
    /// The word will be evaluated against the [solution](Game::solution) of the [Game].
    /// A [GuessResponse] will be formulated, showing us which letters are correctly placed, in the
    /// solution, or just wrong.
    ///
    /// # Errors
    ///
    /// This function will return an error if the length of the [Word] is wrong It will also error
    /// if the game is finished.
    pub fn guess(&mut self, guess: Word) -> GameResult<GuessResponse> {
        if guess.len() != self.length {
            return Err(GameError::GuessHasWrongLength(guess.len()));
        }
        if self.finished || self.step > self.max_steps {
            return Err(GameError::TryingToPlayAFinishedGame);
        }
        if self.wordlist.get_word(&guess).is_none() {
            return Err(GameError::WordNotInWordlist(guess));
        }
        self.step += 1;

        let mut compare_solution = self.solution.0.clone();
        let mut evaluation = Vec::new();
        let mut status: Status;
        for (idx, c) in guess.chars().enumerate() {
            if compare_solution.chars().nth(idx) == Some(c) {
                status = Status::Matched;
                compare_solution.replace_range(idx..idx + 1, "_");
            } else if compare_solution.contains(c) {
                status = Status::Exists;
                compare_solution = compare_solution.replacen(c, "_", 1);
            } else {
                status = Status::None
            }
            evaluation.push((c, status));
        }

        let response = GuessResponse::new(guess, evaluation, self);
        self.responses.push(response.clone());
        self.finished = response.finished();
        Ok(response)
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn solution(&self) -> &WordData {
        &self.solution
    }

    pub fn step(&self) -> usize {
        self.step
    }

    pub fn finished(&self) -> bool {
        self.finished
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
/// let game: Game<_> = GameBuilder::new(&wl)
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
}

impl<'wl, WL: WordList> GameBuilder<'wl, WL> {
    /// make a new [GameBuilder]
    ///
    /// We need a [WordList], so provide one here.
    pub fn new(wl: &'wl WL) -> Self {
        Self {
            length: super::DEFAULT_WORD_LENGTH,
            precompute: false,
            max_steps: super::DEFAULT_MAX_STEPS,
            wordlist: wl,
        }
    }

    /// build a [`Game`] with the stored configuration
    pub fn build(&'wl self) -> GameResult<Game<'wl, WL>> {
        debug!("{:#?}", self);
        let game: Game<WL> =
            Game::build(self.length, self.precompute, self.max_steps, self.wordlist)?;
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
