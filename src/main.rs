use std::io;
use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    let num = rand::rng().random_range(1..=100);
    println!("{num} btw");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed");

    println!("you guessed {guess}")
}
