use core::panic;
use std::fmt::Display;

use crate::error::*;
use crate::wlist::word::{Word, WordData};
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
    ///
    /// # Parameters
    ///
    /// `length` - how many chars the solution has
    /// `precompute` -  how many chars the solution has
    /// `max_steps` -  how many tries the player has
    /// `precompute` -  how many chars the solution has
    /// `wlist` -  which wordlist to use
    /// `generate_solution` -  should the game have a randomly generated solution?

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
    pub fn guess(&mut self, guess: &Word, eval: Option<Evaluation>) -> GameResult<GuessResponse> {
        if guess.len() != self.length {
            return Err(GameError::GuessHasWrongLength(guess.len()));
        }
        if self.finished() || self.step > self.max_steps {
            return Err(GameError::TryingToPlayAFinishedGame);
        }
        if self.wordlist.get_word(&guess).is_none() {
            return Err(GameError::WordNotInWordlist(guess.to_string()));
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

    /// Generates an [Evaluation] for a given solution and guess.
    pub(crate) fn evaluate(solution: WordData, guess: &Word) -> Evaluation {
        let solution = solution.0;
        let mut evaluation: Vec<(char, Status)> = vec![('!', Status::None); solution.len()];
        let mut status: Status;
        let mut buf: Vec<char> = solution.chars().collect();

        #[cfg(debug_assertions)]
        let buflen = solution.len();
        #[cfg(debug_assertions)]
        {
            assert_eq!(buflen, buf.len());
            assert_eq!(buflen, evaluation.len());
        }

        // first the correct solutions
        for ((idx, c_guess), c_sol) in guess.chars().enumerate().zip(solution.chars()) {
            if c_guess == c_sol {
                status = Status::Matched;
                buf[idx] = '!';
                evaluation[idx] = (c_guess, status);
            }
        }

        #[cfg(debug_assertions)]
        {
            assert_eq!(buflen, buf.len());
            assert_eq!(buflen, evaluation.len());
        }

        // then check if the char exists, but was not guessed to be at the correct position
        //
        // We split this up, because finding the "exists" chars at the same time as the "correct"
        // chars causes bugs
        for ((idx, c_guess), c_sol) in guess.chars().enumerate().zip(solution.chars()) {
            if c_guess == c_sol {
                continue;
            } else if buf.contains(&c_guess) {
                status = Status::Exists;
                // replace that char in the buffer to signal that is has been paired with the
                // current char
                let idx_of_a_match = buf
                    .iter()
                    .position(|c| *c == c_guess)
                    .expect("did not find a character in a string even though we know it exists");
                buf[idx_of_a_match] = '!';
            } else {
                status = Status::None;
            }
            evaluation[idx] = (c_guess, status);
        }

        #[cfg(debug_assertions)]
        {
            assert_eq!(buflen, buf.len());
            assert_eq!(buflen, evaluation.len());
        }
        evaluation.into()
    }

    /// discard the last n responses
    pub fn undo(&mut self, n: usize) -> WResult<()> {
        self.responses
            .drain(self.responses.len() - n..self.responses.len());
        Ok(())
    }

    /// get how many characters the words have for this game
    pub fn length(&self) -> usize {
        self.length
    }

    /// get the solution for this game, if the game is aware of one.
    ///
    /// Consider that games may also be played on other platforms, so the game might not "know" the
    /// solution yet.
    pub fn solution(&self) -> Option<&WordData> {
        self.solution.as_ref()
    }

    /// get how many guesses have been made already
    pub fn step(&self) -> usize {
        self.step
    }

    /// true if the game has finished and no more guesses can be made
    pub fn finished(&self) -> bool {
        if self.responses().is_empty() {
            return false;
        }
        self.responses().last().unwrap().finished()
    }

    /// true if the game has finished and the solution was found
    pub fn won(&self) -> bool {
        if !self.finished() || self.responses().is_empty() {
            return false;
        }
        self.responses().last().unwrap().won()
    }

    /// get how many tries the player has
    pub fn max_steps(&self) -> usize {
        self.max_steps
    }

    /// get the responses that were already made
    pub fn responses(&self) -> &Vec<GuessResponse> {
        &self.responses
    }

    /// get the most recent response
    pub fn last_response(&self) -> Option<&GuessResponse> {
        self.responses().last()
    }

    /// get the [WordList] for this game
    pub fn wordlist(&self) -> &WL {
        self.wordlist
    }

    /// get the [Words](Word) that have already been tried
    pub(crate) fn made_guesses(&self) -> Vec<&Word> {
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
#[derive(Debug, Clone, PartialEq)]
pub struct GameBuilder<'wl, WL: WordList> {
    length: usize,
    precompute: bool,
    max_steps: usize,
    wordlist: &'wl WL,
    generate_solution: bool,
    solution: Option<WordData>,
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
            solution: None,
        }
    }

    /// build a [`Game`] with the stored configuration
    pub fn build(&'wl self) -> GameResult<Game<'wl, WL>> {
        trace!("{:#?}", self);
        let mut game: Game<WL> = Game::build(
            self.length,
            self.precompute,
            self.max_steps,
            self.wordlist,
            self.generate_solution,
        )?;
        if self.solution.is_some() {
            game.set_solution(self.solution.clone())
        }
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

    /// Enable or disable Generation of a solution for this builder
    ///
    /// Default is true
    pub fn generate_solution(mut self, generate: bool) -> Self {
        self.generate_solution = generate;
        self
    }

    /// Set the solution for the games built by the builder
    ///
    /// If this is [Some], then the solution generated by
    /// [generate_solution](Self::generate_solution) will be overwritten (if it
    /// is true).
    ///
    /// If [generate_solution](Self::generate_solution) is false and this method is not used, the
    /// game will not have a predetermined solution and will not be able to generate evaluations
    /// for guesses, so these will need to be added manually by the user. The intention is that
    /// this can be used for use cases where the user plays wordle not within wordle-analyzer but
    /// in another program (like their browser). It can also be used to test solvers.
    pub fn solution(mut self, solution: Option<WordData>) -> Self {
        self.solution = solution;
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
        for s in self.responses() {
            write!(f, "\"{s}\",")?;
        }
        Ok(())
    }
}
