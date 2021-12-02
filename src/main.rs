mod utils;
mod solutions;

use std::io;

const MIN_CHOICE: i32 = 1;
const MAX_CHOICE: i32 = 2;

fn main() {
    solutions::run(make_selection());
}

fn make_selection() -> i32 {
    println!("Select a problem ({}-{}):", MIN_CHOICE, MAX_CHOICE);
    let mut line = String::new();
    loop {
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                if let Ok(choice) = line.trim().parse::<i32>() {
                    if MIN_CHOICE <= choice && choice <= MAX_CHOICE {
                        return choice;
                    } else {
                        println!("Invalid number");
                    }
                } else {
                    println!("Please enter a number")
                }
            }
            Err(error) => println!("Error reading input: {}", error),
        }
    } 
}