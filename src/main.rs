#![feature(drain_filter)]

mod utils;
mod solutions;

use std::io;
use solutions::{make_choice_string, SOLVED_PROBLEMS};

fn main() {
    solutions::run(make_selection());
}

fn make_selection() -> i32 {
    println!("Select a problem ({}):", make_choice_string());
    let mut line = String::new();
    loop {
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                if let Ok(choice) = line.trim().parse::<i32>() {
                    if SOLVED_PROBLEMS.contains_key(&choice) {
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
        line.clear();
    } 
}