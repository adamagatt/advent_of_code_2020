#![feature(array_windows)] // Solution 1
#![feature(drain_filter)] // Solution 22
#![feature(result_into_ok_or_err)] // Solution 23

mod utils;
mod solutions;

use std::io;
use solutions::{make_choice_string, SOLVED_PROBLEMS};

fn main() {
    println!("Select a problem ({}, [a]ll):", make_choice_string());
    let mut line = String::new();
    loop {
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line.trim().eq_ignore_ascii_case("a") || line.trim().eq_ignore_ascii_case("all") {
                    SOLVED_PROBLEMS.iter().for_each(
                        |(idx, solution)| {
                            println!("Solution {}:", idx);
                            solution();
                            println!();
                        }
                    );
                    return;
                } else if let Ok(choice) = line.trim().parse::<i32>() {
                    if SOLVED_PROBLEMS.contains_key(&choice) {
                        solutions::run(choice);
                        return;
                    } else {
                        println!("Invalid number");
                    }
                } else {
                    println!("Please enter a number or \"all\"")
                }
            }
            Err(error) => println!("Error reading input: {}", error),
        }
        line.clear();
    } 
}