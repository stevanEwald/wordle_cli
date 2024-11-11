use colored::{ColoredString, Colorize};
use std::fmt::Display;

use super::Error;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LetterState {
    NotGuessed,
    NotInWord,
    IncorrectPosition,
    CorrectPosition,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LetterWithState {
    letter: char,
    state: LetterState,
}
impl LetterWithState {
    pub fn new(letter: char, state: LetterState) -> Result<Self, Error> {
        let letter = letter.to_ascii_lowercase();
        return match letter.is_ascii_alphabetic() {
            true => Ok(Self { letter, state }),
            false => Err(Error::NonLetterChar { character: letter }),
        };
    }
    pub fn colorize(&self) -> ColoredString {
        use LetterState::*;
        let letter = self.letter.to_string();
        match self.state {
            NotGuessed => letter.white(),
            NotInWord => letter.black(),
            IncorrectPosition => letter.yellow(),
            CorrectPosition => letter.green(),
        }
    }
    pub fn state(&self) -> LetterState {
        return self.state;
    }
    pub fn update_state(&mut self, new_state: LetterState) {
        match self.state {
            LetterState::NotGuessed => self.state = new_state,
            LetterState::IncorrectPosition => {
                if new_state == LetterState::CorrectPosition {
                    self.state = LetterState::CorrectPosition
                }
            }
            LetterState::CorrectPosition | LetterState::NotInWord => (),
        }
    }
    pub fn letter(&self) -> char {
        return self.letter;
    }
}
impl Display for LetterWithState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.colorize())
    }
}
