use std::io;
// import io;
// #include <stdio>

fn main() {
    println!("Please enter a number:");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read number!");

    let user_input = user_input.trim();

    let parsed_number = user_input.parse::<i32>();

    println!("You entered: {user_input}, as number: {parsed_number:?}");
}
