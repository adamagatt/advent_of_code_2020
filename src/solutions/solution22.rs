use crate::utils::read_string_lines;
use itertools::iproduct;
use regex::Regex;

use std::{ops::RangeInclusive, cmp::{max, min}};

pub fn solution22 () {
    let commands = parse_commands(&read_string_lines("src/data/solution22.txt"));
    println!("{}", solution22a(&commands));
    println!("{}", solution22b(&commands));
}

fn solution22a(commands: &[Command]) -> usize {
    let mut region = [[[false; 101]; 101]; 101];
    let clip_to_region = |range: CoordRange| range.intersect_ranges(&CoordRange(-50..=50)).0;
    let add_offset = |input: i32| (input + 50) as usize; 

    commands.iter().for_each(|command| {
        let assign_value = command.instruction == Instruction::On;
        iproduct!(
            clip_to_region(command.region.x_range.clone()),
            clip_to_region(command.region.y_range.clone()),
            clip_to_region(command.region.z_range.clone())
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

fn solution22b(commands: &[Command]) -> u128 {    
    let mut active_regions = Vec::<Cube>::new();

    commands.iter().for_each(|command| {
        let mut new_cubes: Vec<Cube> = active_regions
            // Remove any intersected regions for re-processing
            .drain_filter(|region| region.intersects(&command.region))
            // Subtract the current command region from any intersected regions
            .flat_map(|region| {
                let intersection = region.intersection(&command.region);
                if intersection.contains(&region) {
                    // Intersection might completely contain current region
                    vec!()
                } else {
                    // Return parts of the region outside the intersection cube
                    region.subtract_intersection(&intersection)
                } 
            })
            // Clean up by filtering out any sub-cubes that are zero-area
            .filter(|region| !region.is_empty())
            // Add the current command region to the list if it is an ON instruction
            .chain(if command.instruction == Instruction::On {vec!(command.region.clone())} else {vec!()})
            .collect();

        // The filtered regions vector has new and re-processed cubes added to the end
        active_regions.append(&mut new_cubes);
    });

    active_regions.iter()
        .map(|region| region.size())
        .sum()
}

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
            region: Cube {
                x_range: CoordRange(ranges[0]..=ranges[1]),
                y_range: CoordRange(ranges[2]..=ranges[3]),
                z_range: CoordRange(ranges[4]..=ranges[5])
            }
        })
        .collect()
}

#[derive(Debug)]
struct Command {
    instruction: Instruction,
    region: Cube
}

#[derive(Debug, Clone)]
struct Cube{
    x_range: CoordRange,
    y_range: CoordRange,
    z_range: CoordRange,
}

impl Cube {
    fn is_empty(&self) -> bool {
        self.x_range.is_empty() ||
        self.y_range.is_empty() ||
        self.z_range.is_empty()
    }

    fn size(&self) -> u128 {
        vec!(&self.x_range, &self.y_range, &self.z_range).iter()
            .map(|range| (range.0.end() - range.0.start() + 1) as u128)
            .product()
    }

    fn contains(&self, other: &Self) -> bool {
        self.x_range.contains(&other.x_range) &&
        self.y_range.contains(&other.y_range) &&
        self.z_range.contains(&other.z_range)
    }

    fn intersection(&self, other: &Self) -> Self {
        Self {
            x_range: self.x_range.intersect_ranges(&other.x_range),
            y_range: self.y_range.intersect_ranges(&other.y_range),
            z_range: self.z_range.intersect_ranges(&other.z_range)
        }
    }

    fn intersects(&self, other: &Self) -> bool {
        let intersection = self.intersection(other);
        !intersection.x_range.is_empty() &&
        !intersection.y_range.is_empty() &&
        !intersection.z_range.is_empty()
    }

    fn subtract_intersection(&self, intersection: &Self) -> Vec<Self> {
        let mut subcubes = Vec::<Self>::new();

        iproduct!(
            self.x_range.range_segments(&intersection.x_range),
            self.y_range.range_segments(&intersection.y_range),
            self.z_range.range_segments(&intersection.z_range)
        )
            .filter(|(x_range, y_range, z_range)| {
                !x_range.equal_ranges(&intersection.x_range) ||
                !y_range.equal_ranges(&intersection.y_range) ||
                !z_range.equal_ranges(&intersection.z_range)
            })
            .for_each(|(x_range, y_range, z_range)| {
                subcubes.push(Cube { x_range, y_range, z_range });
            });
        subcubes
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {On, Off}

#[derive(Clone, Debug)]
struct CoordRange(RangeInclusive<i32>);

impl CoordRange {
    fn is_empty(&self) -> bool { self.0.is_empty() }

    fn intersect_ranges(&self, other: &Self) -> Self {
        Self (
            max(*self.0.start(), *other.0.start())..=min(*self.0.end(),*other.0.end())
        )
    }

    fn equal_ranges(&self, other: &Self) -> bool {
        *self.0.start() == *other.0.start()  && *self.0.end() == *other.0.end()
    }

    fn range_segments(&self, intersection: &Self) -> Vec<Self> {
        vec!(
            *self.0.start()..=(*intersection.0.start()-1),
            intersection.0.clone(),
            (*intersection.0.end()+1)..=*self.0.end()
        ).into_iter()
            .filter(|range| *range.end() >= *range.start())
            .map(CoordRange)
            .collect()
    }

    fn contains(&self, other: &Self) -> bool {
        self.0.start() <= other.0.end() && self.0.end() < other.0.start()
    }
}