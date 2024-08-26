use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to the guessing game!");

    loop {
        let (min_num, max_num) = loop {
            let min_num = get_input::<u32>("What should the min number be?");
            let max_num = get_input::<u32>("What should the max number be?");

            if min_num <= max_num {
                break (min_num, max_num);
            } else {
                println!("Invalid range! The minimum number must be less than or equal to the maximum number.");
            }
        };

        let secret_number = rand::thread_rng().gen_range(min_num..=max_num);
        let mut attempts: u32 = 0;
        let mut min_guess: u32 = min_num;
        let mut max_guess: u32 = max_num;

        loop {
            let guess: u32 = get_input("Please input your guess.");

            println!("You guessed {guess}");

            attempts += 1;

            if guess < secret_number {
                min_guess = min_guess.max(guess + 1);
                let hint = get_hint(min_guess, max_guess);
                println!("Too small! Here's a possible guess: {hint}");
            } else if guess > secret_number {
                max_guess = max_guess.min(guess - 1);
                let hint = get_hint(min_guess, max_guess);
                println!("Too big! Here's a possible guess: {hint}");
            } else {
                println!("You win! It took you {attempts} guesses!");
                break;
            }
        }

        let play_again: char = get_input("Would you like to play again? [y/n]");
        if play_again != 'y' {
            println!("Ok. Goodbye!!!");
            break;
        }
        println!("Alright, let's play again! Good luck!!");
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        let mut buffer = String::new();
        println!("{prompt}");
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line!");

        match buffer.trim().parse::<T>() {
            Ok(val) => return val,
            Err(_) => {
                println!("Please enter a valid number.");
                buffer.clear();
            }
        }
    }
}

fn get_hint(min: u32, max: u32) -> u32 {
    (min + max) / 2
}
