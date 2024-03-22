use crate::{
    error::WResult,
    game::{response::*, Game},
    wlist::{word::WordData, WordList},
};

pub trait Solver<'wl, WL: WordList>: Clone + Default {
    fn build(wordlist: WL) -> WResult<Self>;
    fn build_game(&self) -> Game<'wl, WL>;
    fn play(game: &mut Game<'wl, WL>) -> Game<'wl, WL>;
    fn solve(game: &mut Game<'wl, WL>) -> WResult<WordData>;
}
