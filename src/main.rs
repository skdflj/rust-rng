use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let num = rand::rng().random_range(1..=100);
    println!("{num} btw");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number");
                continue;
            },
        };

        match guess.cmp(&num) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("you got it !!");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
