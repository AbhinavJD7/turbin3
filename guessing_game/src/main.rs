use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    println!("Saved your guess now the machine is guessing...");

    let random_number = rand::thread_rng().gen_range(1..=100);

    println!("The number was: {random_number}");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    if guess == random_number {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}
