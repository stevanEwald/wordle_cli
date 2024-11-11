mod guess;
mod letter;

use guess::Guess;
use letter::{LetterWithState, LetterState};

use thiserror::Error;
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Hash)]
 pub struct Game {
    target_word: String,
    guesses: Vec<Guess>,
    letters: Vec<LetterWithState>,
    turn_count: usize,
}
impl Game {
    pub fn new(target_word: &str, turn_count: usize) -> Result<Self, Error> {
        let target_word = target_word.trim();
        if target_word.len() != 5 {
            return Err(Error::WrongGuessLength {
                guess: target_word.to_string(),
            });
        }
        let target_word = target_word.to_ascii_lowercase();
        if let Some(character) = target_word.chars().find(|c| !c.is_ascii_alphabetic()) {
            return Err(Error::NonLetterChar { character });
        }
        let letters = ('a'..='z')
            .map(|letter| LetterWithState::new(letter, LetterState::NotGuessed).unwrap())
            .collect();
        return Ok(Game {
            target_word: target_word.to_owned(),
            guesses: Vec::new(),
            letters,
            turn_count,
        });
    }
    pub fn guess(&mut self, word: &str) -> Result<(), Error> {
        if self.turn_number() >= self.turn_count {
            return Err(Error::OutOfTurns {
                target_word: self.target_word.clone(),
            });
        }
        let guess = word.trim().to_ascii_lowercase();
        let new_guess = Guess::new(&guess, &self.target_word)?;

        for guess_letter in new_guess.letters().iter() {
            let game_letter = self.get_mut_letter(guess_letter.letter());
            game_letter.update_state(guess_letter.state());
        }
        self.guesses.push(new_guess);
        if guess == self.target_word {
            return Err(Error::GameWon);
        }
        if self.turn_number() >= self.turn_count {
            return Err(Error::OutOfTurns {
                target_word: self.target_word.clone(),
            });
        }
        return Ok(());
    }
    pub fn turn_number(&self) -> usize {
        return self.guesses.len();
    }
    fn get_mut_letter(&mut self, letter: char) -> &mut LetterWithState {
        return self
            .letters
            .iter_mut()
            .find(|l| l.letter() == letter)
            .unwrap()
    }
}
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.turn_count {
            match self.guesses.get(i) {
                Some(guess) => writeln!(f, "{guess}")?,
                None => writeln!(f)?,
            }
        }

        let keyboard_positions = [
            vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
            vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
            vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'],
        ];
        for row in keyboard_positions.into_iter() {
            let letter_row = row
                .iter()
                .map(|c| {
                    self.letters
                        .iter()
                        .find(|letter| letter.letter() == *c)
                        .unwrap()
                        .to_string()
                })
                .collect::<Vec<_>>()
                .join(" ");
            writeln!(f, "{letter_row}")?;
        }
        return Ok(());
    }
}


#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid Guess: {character}, guess must include only letters")]
    NonLetterChar { character: char },
    #[error("Inavlid Guess: {guess}, guess must be exactly 5 letters")]
    WrongGuessLength { guess: String },
    #[error("Game over: the word was {target_word}")]
    OutOfTurns { target_word: String },
    #[error("Congratulations, You win")]
    GameWon,
}
