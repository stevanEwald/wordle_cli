mod guess;
mod letter;

use guess::Guess;
use letter::{LetterState, LetterWithState};

use std::fmt::Display;
use thiserror::Error;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Game {
    target_word: String,
    guesses: Vec<Guess>,
    letters: Vec<LetterWithState>,
    turn_count: usize,
    state: GameState,
}
impl Game {
    pub fn new(target_word: &str, turn_count: usize) -> Result<Self, Error> {
        let target_word = target_word.trim().to_ascii_lowercase();

        if target_word.len() != 5 {
            return Err(Error::WrongGuessLength {
                guess: target_word.to_string(),
            });
        }

        if let Some(character) = target_word.chars().find(|c| !c.is_ascii_alphabetic()) {
            return Err(Error::NonLetterChar { character });
        }

        let letters = ('a'..='z')
            .map(|letter| LetterWithState::new(letter, LetterState::NotGuessed).unwrap())
            .collect();

        return Ok(Game {
            guesses: Vec::new(),
            state: GameState::InProgress,
            target_word,
            letters,
            turn_count,
        });
    }

    pub fn guess(&mut self, word: &str) -> Result<GameState, Error> {
        //return early if game is already won, or we are out of turns
        match self.state {
            GameState::InProgress => (),
            GameState::OutOfTurns => {
                return Err(Error::OutOfTurns {
                    target_word: self.target_word.clone(),
                })
            }
            GameState::Won => return Err(Error::GameAlreadyWon),
        }
        //create new guess struct from word
        let word = word.trim().to_ascii_lowercase();
        let guess = Guess::new(&word, &self.target_word)?;

        // set new states in self.letters for each letter in guess
        for guess_letter in guess.letters().iter() {
            let game_letter = self
                .get_mut_letter(guess_letter.letter())
                .expect(&format!("character {} not in game letters", guess_letter.letter()));
            game_letter.update_state(guess_letter.state());
        }

        self.guesses.push(guess);

        //update game state if game is over and return the state
        if self.is_won() {
            self.state = GameState::Won;
            return Ok(GameState::Won);
        }
        if self.turn_number() > self.turn_count {
            self.state = GameState::OutOfTurns;
            return Ok(GameState::OutOfTurns);
        }
        return Ok(self.state);
    }

    pub fn is_won(&self) -> bool {
        let last_guess = self.guesses.iter().last();
        return match last_guess {
            Some(last_guess) => last_guess.to_string() == self.target_word,
            None => false,
        };
    }

    pub fn turn_number(&self) -> usize {
        return self.guesses.len() + 1;
    }

    pub fn target_word(&self) -> String {
        return self.target_word.clone();
    }

    fn get_mut_letter(&mut self, letter: char) -> Option<&mut LetterWithState> {
        return self.letters.iter_mut().find(|l| l.letter() == letter);
    }

    fn get_letter(&self, letter: char) -> Option<&LetterWithState> {
        return self.letters.iter().find(|l| l.letter() == letter);
    }
}
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // print previous guesses leaving space for all turns
        for i in 0..self.turn_count {
            match self.guesses.get(i) {
                Some(guess) => writeln!(f, "{}", guess.to_colored_string())?,
                None => writeln!(f)?,
            }
        }
        // define letter layout
        let keyboard_positions = [
            vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
            vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
            vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'],
        ];
        // print letters in layout defined by keyboard_positions
        for row in keyboard_positions.into_iter() {
            let letter_row = row
                .into_iter()
                .map(|c| {
                    self
                        .get_letter(c)
                        .expect(&format!("character {} not in game letters", c))
                        .to_colored_string()
                })
                .collect::<Vec<_>>()
                .join(" ");

            writeln!(f, "{letter_row}")?;
        }
        return Ok(());
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Won,
    OutOfTurns,
    InProgress,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid character: \"{character}\", guess must include only letters")]
    NonLetterChar { character: char },
    #[error("Inavlid guess length: {guess}, guess must be exactly 5 letters")]
    WrongGuessLength { guess: String },
    #[error("Error: tried to guess after game was out of turns")]
    OutOfTurns { target_word: String },
    #[error("Error: tried to guess after game was already won")]
    GameAlreadyWon,
}
