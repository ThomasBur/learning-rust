use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("G'day, and welcome to Guess the Number!");
    let maximum: u32 = 100;
    
    let secret_number = rand::thread_rng().gen_range(1..=maximum);

    let mut num_guesses = 0;
    let max_guessses = (maximum as f32).log2() as u32;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Sorry mate, didn't understand that one.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        num_guesses += 1;
        if num_guesses > max_guessses {
            println!("Sorry mate, too many guesses!");
            break;
        }
    }
}
