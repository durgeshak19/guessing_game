use rand::Rng;
use std::io;
fn main() {
    println!("Guess a number from 1 to 10.");
    let mut name = String::new();

    let _guess: u32 = rand::rng().random_range(1..10);

    let mut count: u32 = 1;
    let max_attempts: u32 = 5;
    loop {
        loop {
            println!(
                "Attempt number: {count}/{max_attempts} ! Guess the number or type exit() to exit"
            );
            io::stdin()
                .read_line(&mut name)
                .expect("Failed to read line");

            name = name.trim().to_string();
            if name.is_empty() {
                println!("Please enter a valid number.");
                continue;
            }
            if name == "exit()" {
                println!("Exiting the game. The correct number was {}", _guess);
                break;
            }
            if name != _guess.to_string() {
                println!("Wrong guess, try again!");
            } else {
                println!(
                    "Congratulations! You guessed the correct number in {} attempts.",
                    count
                );
                break;
            }

            if count == max_attempts {
                println!(
                    "You have used all your attempts. The correct number was {}",
                    _guess
                );
                return;
            }
            count += 1;
            // println!("Your guess is {name} but guess is {_guess}");
            name.clear();
        }
        let mut play_again = String::new();
        println!("Do you want to play again? (yes/no): ");
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if play_again.trim().to_lowercase() == "no" || play_again.trim().to_lowercase() == "n" {
            break;
        } else if play_again.trim().to_lowercase() == "yes"
            || play_again.trim().to_lowercase() == "y"
        {
            name.clear();
            count = 1;
            let _guess: u32 = rand::rng().random_range(1..10);
            continue;
        } else {
            println!("Invalid input. Exiting the game.");
            break;
        }
    }
    println!("Thank you for playing! Goodbye!!");
}
