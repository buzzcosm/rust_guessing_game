use rand::{self, RngExt};
use std::cmp::Ordering;
use std::io;

fn parse_guess(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.trim().parse()
}

fn compare(guess: u32, secret: u32) -> Ordering {
    guess.cmp(&secret)
}

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::rng().random_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("Input your guess:");

        // Variable shadowing: declares a new binding with the same name but a different type.
        // The old binding becomes inaccessible once the new `let` takes effect.
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        // The right-hand side still evaluates using the old String binding (input.trim().parse()),
        // then the result is bound to a new u32 variable also named `guess`, shadowing the String.
        let guess = match parse_guess(&input) {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        attempts += 1;
        println!("You guessed: {}", guess);

        match compare(guess, secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed correctly after {} attempts!", attempts);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_number() {
        assert_eq!(parse_guess("42"), Ok(42));
    }

    #[test]
    fn parse_trims_whitespace() {
        assert_eq!(parse_guess("  7\n"), Ok(7));
    }

    #[test]
    fn parse_invalid_input() {
        assert!(parse_guess("qwer").is_err());
    }

    #[test]
    fn parse_negative_rejected() {
        assert!(parse_guess("-5").is_err());
    }

    #[test]
    fn compare_too_small() {
        assert_eq!(compare(3, 10), Ordering::Less);
    }

    #[test]
    fn compare_too_big() {
        assert_eq!(compare(99, 10), Ordering::Greater);
    }

    #[test]
    fn compare_correct() {
        assert_eq!(compare(10, 10), Ordering::Equal);
    }
}
