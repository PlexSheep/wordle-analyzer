use crate::wlist::WordList;

use super::Game;

pub struct Summary<'wl, WL: WordList> {
    data: Vec<&'wl Game<'wl, WL>>,
}

impl<'wl, WL: WordList> Summary<'wl, WL> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn push(&mut self, game: &'wl Game<WL>) {
        self.data.push(game)
    }
    pub fn pop(&mut self) -> Option<&Game<WL>> {
        self.data.pop()
    }
}

impl<'wl, WL: WordList> From<Vec<&'wl Game<'wl, WL>>> for Summary<'wl, WL> {
    fn from(value: Vec<&'wl Game<'wl, WL>>) -> Self {
        Summary { data: value }
    }
}

impl<'wl, WL: WordList> From<Vec<&'wl mut Game<'wl, WL>>> for Summary<'wl, WL> {
    fn from(value: Vec<&'wl mut Game<'wl, WL>>) -> Self {
        // looks weird, but is ok
        let value: Vec<&'wl Game<'wl, WL>> = value.into_iter().map(|v| &*v).collect();
        Summary { data: value }
    }
}

impl<'wl, WL: WordList> From<&'wl mut std::vec::Vec<Game<'wl, WL>>> for Summary<'wl, WL> {
    fn from(value: &'wl mut std::vec::Vec<Game<'wl, WL>>) -> Self {
        let value: Vec<&'wl Game<'wl, WL>> = value.iter().collect();
        Summary { data: value }
    }
}

impl<'wl, WL: WordList> From<&'wl std::vec::Vec<Game<'wl, WL>>> for Summary<'wl, WL> {
    fn from(value: &'wl std::vec::Vec<Game<'wl, WL>>) -> Self {
        let value: Vec<&'wl Game<'wl, WL>> = value.iter().collect();
        Summary { data: value }
    }
}
