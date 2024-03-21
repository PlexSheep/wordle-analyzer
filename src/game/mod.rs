#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Game {
    length: usize,
    precompute: bool,
    max_steps: usize,
    step: usize,
    solution: String,
}

impl Game {
    /// get a new [`GameBuilder`]
    pub fn builder() -> GameBuilder {
        GameBuilder::default()
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
    pub(crate) fn build(length: usize, precompute: bool, max_steps: usize) -> anyhow::Result<Self> {
        let _game = Game {
            length,
            precompute,
            max_steps,
            step: 0,
            solution: String::default(), // we actually set this later
        };

        // TODO: load wordlist of possible answers
        // TODO: select one as a solution at random
        // NOTE: The possible answers should be determined with a wordlist that has the
        // frequencies/probabilities of the words. We then use a sigmoid function to determine if a
        // word can be a solution based on that value. Only words above some threshold of
        // commonness will be available as solutions then. Next, we choose one of the allowed words
        // randomly.
        todo!();
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
/// # use anyhow::Result;
/// # fn main() -> Result<()> {
/// let game: Game = GameBuilder::default()
///     .build()?;
/// # Ok(())
/// # }
/// ```
///
/// ```
/// # use wordle_analyzer::game::*;
/// # use anyhow::Result;
/// # fn main() -> Result<()> {
/// let game: Game = Game::builder()
///     .length(5)
///     .precompute(false)
///     .max_steps(6)
///     .build()?;
/// # Ok(())
/// # }
/// ```
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameBuilder {
    length: usize,
    precompute: bool,
    max_steps: usize,
}

impl GameBuilder {
    /// build a [`Game`] with the stored configuration
    pub fn build(self) -> anyhow::Result<Game> {
        let game: Game = Game::build(self.length, self.precompute, self.max_steps)?;
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
        self
    }
}

impl Default for GameBuilder {
    fn default() -> Self {
        Self {
            length: super::DEFAULT_WORD_LENGTH,
            precompute: false,
            max_steps: super::DEFAULT_MAX_STEPS,
        }
    }
}
