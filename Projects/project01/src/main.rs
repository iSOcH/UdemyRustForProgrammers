use colored::*;
use rand::Rng;
use std::io;

fn main() {
    let secret_num = rand::thread_rng().gen_range(0..=10);

    println!("Welcome to the Guessing Game!\n");

    loop {
        println!("Please enter a number in the range [0, 10]:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read number!");

        let guessed_num = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(e) => {
                println!("Failed to parse input: {e}");
                continue;
            }
        };

        // analyzer says we should rewrite to match, but this is ugly
        // for the equal case, we cannot match on `guessed_num` since it is simply a binding name, it
        // does _not_ compare to (or care in any way about) the value of the outer binding
        // match guessed_num {
        //     _ if guessed_num == secret_num => {
        //         println!("{}", "Your guess is correct".green());
        //         break;
        //     }
        //     _ if guessed_num > secret_num => {
        //         println!("{}", "Your guessed number is too big!".red());
        //     }
        //     _ => {
        //         println!("{}", "Your guessed number is too small!".red());
        //     }
        // }

        // feels much better
        match guessed_num.cmp(&secret_num) {
            std::cmp::Ordering::Equal => {
                println!("{}", "Your guess is correct".green());
                break;
            }
            std::cmp::Ordering::Greater => println!("{}", "Your guessed number is too big!".red()),
            std::cmp::Ordering::Less => println!("{}", "Your guessed number is too small!".red()),
        };
    }
}
