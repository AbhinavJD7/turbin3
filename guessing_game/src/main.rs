use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number GAME!");

    loop{
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

    match guess.cmp(&random_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!"); 
            break;
        }
     }
    }
}
