# Guessing Game

A small Rust command-line guessing game. The program chooses a random number and asks the player to guess it within a limited number of attempts.

Features
- Random number generation using `rand`.
- Limited number of attempts per round.
- Option to exit mid-game by typing `exit()`.
- Prompt to play again after each round.

Where the code lives
- Main crate(s): `src/main.rs` (this workspace also contains an inner `guessing_game/` crate). See the `src` folder for the implementation.

Prerequisites
- Rust and Cargo installed. If not installed, install via rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

How to run

Run from the project root

```bash
cargo run
```

Usage
- The program will prompt: "Guess a number from 1 to 10." Type a number and press Enter.
- You can type `exit()` to quit early.
- After a round you will be asked whether you want to play again (yes/no).

Examples

- Correct guess:
```
Attempt number: 1/5 ! Guess the number or type exit() to exit
5
Congratulations! You guessed the correct number in 1 attempts.
```

- Exit mid-game:
```
Attempt number: 2/5 ! Guess the number or type exit() to exit
exit()
Exiting the game. The correct number was 7
```

Notes & troubleshooting
- If `cargo run` fails, make sure you are in the correct directory (see the commands above) and that you have the Rust toolchain installed.
- If you encounter network or permission errors when pushing to GitHub, ensure your `git` authentication (SSH key or HTTPS credentials) is set up and that the remote exists.

License
- This repository does not include a license file by default. Add one if you plan to publish or share.

Have fun!
