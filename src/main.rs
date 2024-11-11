use rand::prelude::*;
use std::{fs, io, path::Path};
use wordle::*;
use colored::Colorize;
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
    let mut guess_result: Result<(), wordle::Error> = Ok(());

    loop {
        std::process::Command::new("clear").status().unwrap();
        guess.clear();
        println!("{game}");
        if let Err(e) = &guess_result {
            let colored_error_message = e.to_string().red();
            println!("{colored_error_message}");
            if let wordle::Error::OutOfTurns { .. } | wordle::Error::GameWon = e {
                break;
            }
        }
        stdin.read_line(&mut guess).unwrap();

        guess_result = game.guess(&guess);
    }
}
