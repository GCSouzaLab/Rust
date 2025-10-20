use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Good luck, you have the chance to guess the secret number!");

    let mut guess = String::new();

    // It's needed to add the "import", first line!
    io::stdin() 
    .read_line(&mut guess)
    .expect("Failed to read line.");
    
    let value = guess.trim().parse::<i32>().expect("Please type a number!");
    let correct = value == secret_number;

    if correct {
        println!("You hit");
    } else {
        println!("You loss, the secret number is {secret_number}");
    }
}
