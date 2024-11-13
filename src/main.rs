use colored::Colorize;
use rand::prelude::*;
use std::{fs, io, path::Path};

mod game;
use game::*;
fn main() {
    let word_list_string = std::env::var("CARGO_MANIFEST_DIR")
        .expect("failed to read enviornment variable: CARGO_MAINIFEST_DIR")
        + "/words.txt";
    let word_list_path = Path::new(&word_list_string);
    let word_list = fs::read_to_string(word_list_path).expect("failed to read word list file");

    let target_word = word_list
        .lines()
        .choose(&mut ThreadRng::default())
        .expect("no valid words in file");
    assert_eq!(target_word.len(), 5);

    let turn_count = 6;

    let mut game = Game::new(target_word, turn_count).unwrap();

    let stdin = io::stdin();
    let mut guess = String::new();
    let mut error_message = None;
    loop {
        std::process::Command::new("clear").status().unwrap();
        guess.clear();
        print!("{game}");
        if let Some(error) = error_message.take() {
            println!("{error}")
        }
        stdin.read_line(&mut guess).unwrap();
        match game.guess(&guess) {
            Ok(GameState::InProgress) => (),
            Ok(GameState::Won) => {
                println!("Congratulations, You won");
                break;
            }
            Ok(GameState::OutOfTurns) => {
                println!("Game over: the word was {}", game.target_word());
                break;
            }
            Err(e) => error_message = Some(e.to_string().red()),
        }
    }
}
