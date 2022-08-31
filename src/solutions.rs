use std::collections::BTreeMap;
use lazy_static::lazy_static;
use itertools::Itertools;

mod solution1;
mod solution2;
mod solution3;
mod solution4;
mod solution5;
mod solution6;
mod solution7;
mod solution8;
mod solution9;
mod solution10;
mod solution11;
mod solution12;
mod solution13;
mod solution14;
mod solution15;
mod solution16;
mod solution17;
mod solution18;
mod solution22;
mod solution23;

lazy_static! {
    pub static ref SOLVED_PROBLEMS: BTreeMap<i32, fn() -> ()> = BTreeMap::from([
        (1,  solution1::solution1   as fn()),
        (2,  solution2::solution2   as fn()),
        (3,  solution3::solution3   as fn()),
        (4,  solution4::solution4   as fn()),
        (5,  solution5::solution5   as fn()),
        (6,  solution6::solution6   as fn()),
        (7,  solution7::solution7   as fn()),
        (8,  solution8::solution8   as fn()),
        (9,  solution9::solution9   as fn()),
        (10, solution10::solution10 as fn()),
        (11, solution11::solution11 as fn()),
        (12, solution12::solution12 as fn()),
        (13, solution13::solution13 as fn()),
        (14, solution14::solution14 as fn()),
        (15, solution15::solution15 as fn()),
        (16, solution16::solution16 as fn()),
        (17, solution17::solution17 as fn()),
        (18, solution18::solution18 as fn()),
        (22, solution22::solution22 as fn()),
        (23, solution23::solution23 as fn())
    ]);
}

pub fn run(choice: i32) {
    let solution_function = SOLVED_PROBLEMS
        .get(&choice)
        .unwrap_or_else(|| panic!("Solution {} is not implemented", choice));
    
    solution_function();
}

pub fn make_choice_string() -> String {
    let valid_choices: Vec<&i32> = SOLVED_PROBLEMS.keys().collect();

    let (mut run_starts, mut run_ends) = valid_choices.array_windows()
        .fold(
            (Vec::<&i32>::new(), Vec::<&i32>::new()),
            |(mut run_starts, mut run_ends), &[prev, next]| {
                if (prev + 1) != *next {
                    run_starts.push(next);
                    run_ends.push(prev);
                }
                (run_starts, run_ends)
            }
        );
    run_starts.insert(0, valid_choices[0]);
    run_ends.push(valid_choices[valid_choices.len()-1]);
    
    run_starts.iter().zip(run_ends.iter())
        .map(|(&start, &end)|
            if *start == *end {
                start.to_string()
            } else {
                format!("{}-{}", start, end)
            }
        )
        .join(", ")
}