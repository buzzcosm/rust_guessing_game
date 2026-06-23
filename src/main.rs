use rand::{self, RngExt};
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::rng().random_range(1..=100);
    //println!("The secret number is: {}", secret_number);

    let mut attempts = 0;

    loop {
        println!("Input your guess:");
        // Variable shadowing: declares a new binding with the same name but a different type.
        // The old binding becomes inaccessible once the new `let` takes effect.
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // The right-hand side still evaluates using the old String binding (guess.trim().parse()),
        // then the result is bound to a new u32 variable also named `guess`, shadowing the String.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        attempts += 1;
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed correctly after {} attempts!", attempts);
                break;
            }
        }

    }
}
