# Guessing Game in Rust with Cargo

> REF: [YouTube: Coding your first project in Rust](https://www.youtube.com/watch?v=8tduMGPZsSk)

1. Create a new project with `cargo new guessing_game`
2. Test if all correct by running `cargo run`
3. Add dependencies in `Cargo.toml` for random numbers with `cargo add rand`

## Rust Concepts Demonstrated

| Concept | Where |
|---|---|
| **Variable shadowing** | `guess` is first a `String`, then rebound as `u32` using the same name |
| **Mutability** | `let mut guess` is needed to receive `read_line`; the parsed `u32` is immutable |
| **`loop` / `break`** | Runs indefinitely until the player guesses correctly |
| **`match` + `Ordering`** | Compares the guess to the secret number with three arms: `Less`, `Greater`, `Equal` |
| **`expect`** | Minimal error handling — panics with a message on failure |
| **External crate (`rand`)** | Generates a random number via `rand::rng().random_range(1..=100)` |