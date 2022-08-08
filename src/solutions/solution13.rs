use crate::utils::read_string_lines;

use std::collections::HashSet;

pub fn solution13 () {
    let (dots, instructions) = parse_input(read_string_lines("src/data/solution13.txt"));

    println!("{}", solution13a(&dots, &instructions));
    println!("{}", solution13b(&dots, &instructions));
}

type Dot = (i32, i32);
enum ParseAxis{X, Y}
struct Fold {
    axis: ParseAxis,
    coordinate: i32
}

fn solution13a(dots: &HashSet<Dot>, instructions: &[Fold]) -> usize {
    // Only care about the first instruction for part A
    let instruction = &instructions[0];
    
    let new_dots = dots.iter()
        .map(|dot| position_after_fold(dot, instruction))
        // Collecting into a set will eliminate duplicates
        .collect::<HashSet<Dot>>();

    new_dots.len()
}

fn solution13b(dots: &HashSet<Dot>, instructions: &[Fold]) -> String {
    let new_dots = dots.iter()
        .map(|dot| instructions.iter()
            // Can apply all instructions by folding the entire list of them over
            // the dot's original position
            .fold(*dot,
                |new_dot, instruction| position_after_fold(&new_dot, instruction)
            )
        )
        .collect::<HashSet<Dot>>();

    // Determine the grid size for rendering the dots onto 
    let max_x = new_dots.iter().map(|(x, _y)| x).max().unwrap() + 1;
    let max_y = new_dots.iter().map(|(_x, y)| y).max().unwrap() + 1;
    let mut output = vec![vec![' '; max_x as usize]; max_y as usize];

    // Render all dots on to the grid
    for (dot_x, dot_y) in new_dots {
        output[dot_y as usize][dot_x as usize] = '█';
    }

    // Parse the grid as string output, each line separated by newline characters
    output.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn position_after_fold((dot_x, dot_y): &Dot, fold: &Fold) -> Dot {
    match fold.axis {
        ParseAxis::X => (fold.coordinate - (fold.coordinate - *dot_x).abs(), *dot_y),
        ParseAxis::Y => (*dot_x, fold.coordinate - (fold.coordinate - *dot_y).abs())
    }
}

fn parse_input(lines: Vec<String>) -> (HashSet<Dot>, Vec<Fold>) {
    // Lines until the first empty line are the dot initial coordinates
    let dots = lines.iter()
        .take_while(|line| !line.is_empty())
        .map(|dot_coords| {
            let tokens = dot_coords
                .split(',')
                .collect::<Vec<&str>>();
            (tokens[0].parse::<i32>().unwrap(), tokens[1].parse::<i32>().unwrap())
        })
        .collect::<HashSet<Dot>>();

    // Begin reading after the number of dots until the next empty line
    let folds = lines.iter()
        .skip(dots.len() + 1)
        .take_while(|line| !line.is_empty())
        .map(|instruction| {
            let direction = &instruction[11..12];
            let coordinate = &instruction[13..];
            Fold {
                axis: if direction == "x" {ParseAxis::X} else {ParseAxis::Y},
                coordinate: coordinate.parse::<i32>().unwrap()
            }
        })
        .collect::<Vec<Fold>>();
    
        (dots, folds)
}