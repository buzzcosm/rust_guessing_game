# Guessing Game in Rust with Cargo

> REF: [YouTube: Coding your first project in Rust](https://www.youtube.com/watch?v=8tduMGPZsSk)

1. Create a new project with `cargo new guessing_game`
2. Test if all correct by running `cargo run`
3. Add dependencies in `Cargo.toml` for random numbers with `cargo add rand`

## Run with Docker

1. Build the image: `docker build -t guessing_game .`
2. Run interactively: `docker run -it --rm --init --name guessing_game_container guessing_game`

> `-it` is required — `-i` keeps stdin open, `-t` allocates a TTY for terminal input.
> `--init` is required — without it the Rust process runs as PID 1 and does not receive `Ctrl+C` (SIGINT).

## Run locally with Cargo

1. Build: `cargo build --release`
2. Run: `cargo run`
3. Test: `cargo test`

## Rust Concepts Demonstrated

| Concept | Where |
|---|---|
| **Variable shadowing** | `guess` is first a `String`, then rebound as `u32` using the same name |
| **Mutability** | `let mut guess` is needed to receive `read_line`; the parsed `u32` is immutable |
| **`loop` / `break`** | Runs indefinitely until the player guesses correctly |
| **`match` + `Ordering`** | Compares the guess to the secret number with three arms: `Less`, `Greater`, `Equal` |
| **`expect`** | Minimal error handling — panics with a message on failure |
| **External crate (`rand`)** | Generates a random number via `rand::rng().random_range(1..=100)` |