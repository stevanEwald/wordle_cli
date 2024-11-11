use std::fmt::Display;
use super::letter::{LetterWithState, LetterState};
use super::Error;
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Guess {
    letters: [LetterWithState; 5],
}
impl Guess {
    pub fn new(letters: &str, target_word: &str) -> Result<Self, Error> {
        let letters = letters.trim().to_ascii_lowercase();
        if letters.len() != 5 {
            return Err(Error::WrongGuessLength {
                guess: letters.to_string(),
            });
        }
        let letters: [LetterWithState; 5] = letters
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let letter_state = match target_word.contains(c) {
                    true => match target_word.chars().nth(i).unwrap() == c {
                        true => LetterState::CorrectPosition,
                        false => LetterState::IncorrectPosition,
                    },
                    false => LetterState::NotInWord,
                };
                LetterWithState::new(c, letter_state)
            })
            .collect::<Result<Vec<_>, Error>>()?
            .try_into()
            .unwrap();
        return Ok(Self { letters });
    }
    pub fn letters(&self) -> &[LetterWithState; 5] {
        return &self.letters;
    }
}
impl Display for Guess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guess = self
            .letters
            .iter()
            .map(|l| l.to_string())
            .collect::<String>();
        return write!(f, "{}", guess);
    }
}

