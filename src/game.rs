use std::cmp::Ordering;

pub fn parse_guess(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.trim().parse()
}

pub fn compare(guess: u32, secret: u32) -> Ordering {
    guess.cmp(&secret)
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
