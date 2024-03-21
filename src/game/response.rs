use crate::wlist::word::Word;

pub struct GuessResponse {
    guess: Word,
    status: Vec<(char,Status)>,
    step: usize
}

pub enum Status {
    None,
    Exists,
    Matched
}
