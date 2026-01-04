# Guessing Game

Welcome to the Guessing Game! This is a small Rust command-line application where the player must guess a randomly generated number between 1 and 10.

## Overview

The Guessing Game is intended to be a fun, interactive example for learning Rust. The game prompts you to guess a number, gives feedback when a guess is wrong, and informs you when you've guessed correctly. You have a limited number of attempts per round and can choose to play again after each round.

## Features

- Random number generation between 1 and 10 (uses `rand`).
- User input handling for guesses and simple commands.
- Limited number of attempts per round (default: 5).
- Option to exit mid-game with `exit()` and to play again after each round.

## Getting Started

To run the Guessing Game locally:

1. Install Rust and Cargo (if not already installed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the repository:

```bash
git clone https://github.com/durgeshak19/guessing_game.git
```

3. Change into the crate directory and run:

```bash
cd guessing_game/guessing_game
cargo run
```

Or, from the project root (if you prefer):

```bash
cd /path/to/guessing_game
cargo run -p guessing_game
```

The first time you build it, Cargo will compile dependencies and the program; subsequent runs will be faster.

## How to Play

1. When the program prompts, type a number between 1 and 10 and press Enter.
2. Type `exit()` at any time to quit the current round; the program will show the correct number.
3. After a round ends (win or lose), you'll be asked whether you want to play again (yes/no).

## Example session

```
Attempt number: 1/5 ! Guess the number or type exit() to exit
5
Wrong guess, try again!
Attempt number: 2/5 ! Guess the number or type exit() to exit
3
Congratulations! You guessed the correct number in 2 attempts.
Do you want to play again? (yes/no):
no
Thank you for playing! Goodbye!
```

## Contributing

Contributions are welcome. If you'd like to improve the game (better input parsing, tests, additional features), please open an issue or submit a pull request.

## License

This project is released under the MIT License. If you distribute or publish this repository, consider adding a `LICENSE` file containing the full MIT license text.

## Acknowledgments

Thanks for trying the Guessing Game — have fun and happy coding!