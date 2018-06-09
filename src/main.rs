extern crate rand;

use rand::Rng;
use std::io;
use std::io::prelude::*;
use std::fs::File;

const ALLOWED_ATTEMPTS: u8 = 6;

struct Letter {
    character: char,
    revealed: bool
}

fn main() {
    let mut chances_left = ALLOWED_ATTEMPTS;
    let chosen_word = select_word();
    let mut letters = create_letters(&chosen_word);
    let mut correct_count = 0;
    
    loop {
        println!("You have {} chances left.", chances_left);
        display_progress(&letters);

        println!("Enter your guess:");
        let user_guess = read_input();

        if user_guess == '*' {
            println!("Incorrect input, leaving game.");
            break;
        }
        
        let mut correct_guess = false;

        for letter in letters.iter_mut() {
            if letter.character == user_guess {
                letter.revealed = true;
                correct_guess = true;
                correct_count += 1;
            }
        }

        if !correct_guess {
            chances_left -= 1;
        }

        if chances_left == 0 {
            println!("You lost :(");
            println!("The word was: {}", chosen_word);
            break;
        } else if correct_count == chosen_word.len() {
            println!("YOU WON!!!");
            println!("The word was {}", chosen_word);
            break;
        }
    }
}

fn select_word() -> String {
    let mut file = File::open("words.txt")
      .expect("Could not open file.");

    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents)
      .expect("Could not read file.");
    
    let word_vec: Vec<&str> = file_contents.trim().split(',').collect();

    let rand_idx = rand::thread_rng().gen_range(0, word_vec.len());

    return String::from(word_vec[rand_idx]);
}

fn create_letters(word: &String) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();

    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false
        });
    }

    return letters;
}

fn display_progress(letters: &Vec<Letter>) {
    let mut display = String::from("Progress:");

    for letter in letters {
        display.push(' ');

        if letter.revealed {
            display.push(letter.character);
        } else {
            display.push('_');
        }
    }

    display.push(' ');

    println!("{}", display);
}

fn read_input() -> char {
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => { return c; }
                None => { return '*'; }
            }
        }
        Err(_) => { return '*'; }
    }
        
}