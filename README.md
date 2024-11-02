# Guessing Game in Rust

A fun command-line fruit guessing game built with Rust! This game randomly selects a fruit, and the player has to guess it. It’s a simple demonstration of Rust’s capabilities, including generating random values, handling user input, and implementing a game loop.

## Features

- **Random Fruit Selection**: Each game randomly selects a fruit from a predefined list.
- **User Interaction**: Prompts the player to guess the fruit, providing feedback on each guess.
- **Looped Gameplay**: The game continues until the player guesses the correct fruit.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (Ensure it’s installed by running `rustc --version`)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/agastya3636/guessing_game_in_rust.git
   cd guessing_game_in_rust
   ```

2. Run the game:
   ```bash
   cargo run
   ```

## How to Play

- When the game starts, it will randomly select a fruit from the list.
- Type in your guess and press enter.
- If you guess correctly, you win! Otherwise, the game informs you of the correct answer and allows you to try again.

## Project Structure

- **src/main.rs**: Contains the game logic, including the random fruit selection, input handling, and loop control.

## Future Enhancements

- [ ] Add multiple levels with more fruits.
- [ ] Keep score of the number of attempts.
- [ ] Add hints or provide a limited number of guesses.

