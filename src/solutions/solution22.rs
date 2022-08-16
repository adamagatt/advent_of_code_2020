use crate::utils::read_string_lines;
use itertools::iproduct;
use regex::Regex;

use std::{ops::RangeInclusive, cmp::{max, min}, collections::HashSet};

pub fn solution22 () {
    let commands = parse_commands(&read_string_lines("src/data/solution22.txt"));
    println!("{}", solution22a(&commands));
    println!("{}", solution22b(&commands));
}

fn solution22a(commands: &[Command]) -> usize {
    let mut region = [[[false; 101]; 101]; 101];
    let clip_to_region = |range| intersect_ranges(range, &(-50..=50));
    let add_offset = |input: i32| (input + 50) as usize; 

    commands.iter().for_each(|command| {
        let assign_value = command.instruction == Instruction::On;
        iproduct!(
            clip_to_region(&command.x_range),
            clip_to_region(&command.y_range),
            clip_to_region(&command.z_range)
        ).for_each(|(x, y, z)| {
            region[add_offset(x)][add_offset(y)][add_offset(z)] = assign_value;
        });
    });

    region.iter()
        .flat_map(|x| x.iter())
        .flat_map(|x| x.iter())
        .filter(|&square| *square)
        .count()
}

fn solution22b(commands: &[Command]) -> usize {    
    let mut coords_on = HashSet::new();

    commands.iter().enumerate().for_each(|(idx, command)| {
        dbg!(idx);

        iproduct!(
            command.x_range.clone(),
            command.y_range.clone(),
            command.z_range.clone()
        ).for_each(|(x, y, z)| {
            if command.instruction == Instruction::On {
                coords_on.insert((x, y, z));
            } else {
                coords_on.remove(&(x, y, z));
            }
        });
    });

    coords_on.len()
}

#[derive(Debug)]
struct Command {
    instruction: Instruction,
    x_range: CoordRange,
    y_range: CoordRange,
    z_range: CoordRange,
}

#[derive(Debug, PartialEq)]
enum Instruction {On, Off}

type CoordRange = RangeInclusive<i32>;

fn parse_commands(input_lines: &[String]) -> Vec<Command> {
    let re = Regex::new(r"(on|off).x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)")
        .unwrap();

    let to_coord = |str_in: &str| -> i32 { str_in.parse().unwrap() };

    input_lines.iter()
        .filter_map(|line| re.captures(line))
        .map(|capture| {(            
            if &capture[1] == "on" {Instruction::On} else {Instruction::Off},
            capture.iter()
                .skip(2)
                .flatten()
                .map(|x| to_coord(x.as_str()))
                .collect::<Vec<i32>>()
        )})
        .map(|(instruction, ranges)| Command {
            instruction,
            x_range: RangeInclusive::new(ranges[0], ranges[1]),
            y_range: RangeInclusive::new(ranges[2], ranges[3]),
            z_range: RangeInclusive::new(ranges[4], ranges[5])
        })
        .collect()
}

fn intersect_ranges<T: Ord+Copy>(left: &RangeInclusive<T>, right: &RangeInclusive<T>) -> RangeInclusive<T> {
    max(*left.start(), *right.start())..=min(*left.end(),*right.end())
}