use std::io;
use rand::prelude::*;

fn main() {
    let fruits = ["apple", "banana", "cherry", "date", "elderberry", "fig", "grape"];
    let mut rng = rand::thread_rng();
    loop {
        println!("Welcome to the fruit guessing game!");
        
        let random_index = rng.gen_range(0..fruits.len());
        let chosen_fruit = fruits[random_index];

        println!("Guess the fruit!");
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        if !fruits.contains(&guess.trim().to_lowercase().as_str()) {
            println!("Fruit does not found!");
            
            continue;
        } 
        else if guess.trim().eq_ignore_ascii_case(chosen_fruit) {
            println!("You guessed right!");
            break;
        } else {
            println!("You guessed wrong! The fruit was {}", chosen_fruit);
        }
    }
}
