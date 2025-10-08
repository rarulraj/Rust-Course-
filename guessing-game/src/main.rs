use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number!");

    let s_n =  rand::thread_rng().gen_range(1..=100);

    println!("please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed: {guess}");

    println!("the secret number is: {sN}");
}
