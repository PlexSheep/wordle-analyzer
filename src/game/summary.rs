use crate::wlist::WordList;

use super::Game;

pub struct Summary<'wl, WL: WordList> {
    data: &'wl Vec<Game<'wl, WL>>,
}

impl<'wl, WL: WordList> From<&'wl Vec<Game<'wl, WL>>> for Summary<'wl, WL> {
    fn from(value: &'wl Vec<Game<'wl, WL>>) -> Self {
        Summary { data: value }
    }
}

impl<'wl, WL: WordList> From<&'wl mut Vec<Game<'wl, WL>>> for Summary<'wl, WL> {
    fn from(value: &'wl mut Vec<Game<'wl, WL>>) -> Self {
        Summary { data: value }
    }
}
