use colored::Colorize;
use rand::prelude::*;
use std::{fs, io::{self, Write}, path::Path};

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



    clear_screen();
    print!("{game}");
    loop {
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        let guess_result = game.guess(&guess);

        clear_screen();

        print!("{game}");
        match guess_result {
            Ok(GameState::InProgress) => (),
            Ok(GameState::Won) => {
                println!("Congratulations, You won");
                break;
            }
            Ok(GameState::OutOfTurns) => {
                println!("Game over: the word was {}", game.target_word());
                break;
            }
            Err(e) => println!("{}", e.to_string().red()),
        }
    }
}

fn clear_screen() {
    let escape = "\x1B";
    let clear_screen_code = format!("{escape}[2J",);
    let reset_cursor_code = format!("{escape}[H");
    print!("{clear_screen_code}");
    print!("{reset_cursor_code}");
}