use crate::utils::read_string_lines;
use regex::Regex;
use itertools::Itertools;

pub fn solution17 () {
    let target_area = parse_target_area(&read_string_lines("src/data/solution17.txt").remove(0));
    println!("{}", solution17a(&target_area));
    println!("{}", solution17b(&target_area));
}

fn solution17a(target_area: &TargetArea) -> i32 {
    // Search begins from maximum y velocity down to zero. The maximum
    // considered is the distance between the origin and the bottom of
    // the target area. Velocities larger than this are likely to skip
    // over the entire target area in one step.
    for y_vel in (target_area.min_y..=(-target_area.min_y)).rev() {
        // Search from 0 x velocity to a value that would skip past the
        // target area in a single step
        for x_vel in 0..=(target_area.max_x) {
            if probe_ends_within_area(
                Probe {x_pos: 0, y_pos: 0, x_vel, y_vel},
                target_area
            ) {
                return height_for_initial_y_vel(y_vel);
            }
        }
    }
    panic!("Couldn't find initial velocities that ended up within the target area")
}

fn solution17b(target_area: &TargetArea) -> i32 {
    let mut success_count = 0;

    // Same search criteria as in part A
    // Could also be done as an iterator stream and filter/count
    for y_vel in target_area.min_y..=(-target_area.min_y) {
        for x_vel in 0..=(target_area.max_x) {
            if probe_ends_within_area(
                Probe {x_pos: 0, y_pos: 0, x_vel, y_vel},
                target_area
            ) {
                success_count += 1;
            }
        }
    }
    success_count
}

fn probe_ends_within_area(mut probe: Probe, target_area: &TargetArea) -> bool {
    while !end_simulation(&probe, &target_area) {
        update_probe(&mut probe);
        if target_area.contains(&probe) {
            return true;
        }
    }
    false
}

fn end_simulation(probe: &Probe, target_area: &TargetArea) -> bool {
    // If the probe is now further than the target area
    probe.x_pos > target_area.max_x
    // If the probe is below the target area and descending
    || (probe.y_pos < target_area.min_y && probe.y_vel < 0)
}

fn update_probe(probe: &mut Probe) {
    probe.x_pos += probe.x_vel;
    probe.y_pos += probe.y_vel;
    probe.y_vel -= 1;
    if probe.x_vel > 0 {
        probe.x_vel -= 1;
    }
}

fn height_for_initial_y_vel(y_vel: i32) -> i32 {
    // Total height with motion rules used is triangular quadratic relative to
    // the initial y velocity
    (y_vel * (y_vel + 1)) / 2
}

fn parse_target_area(line_in: &String) -> TargetArea {
    let re = Regex::new(r"(-?\d+)").unwrap();
    let target_area = re.find_iter(line_in)
        // Convert each captured number to i32
        .map(|cap| cap.as_str().parse::<i32>().unwrap())
        // Read 4 numbers and provide together as a single iterator value
        .tuples::<(_, _, _, _)>()
        .map(|bounds| TargetArea{min_x: bounds.0, max_x: bounds.1, min_y: bounds.2, max_y: bounds.3})
        // We only want the first TargetArea constructed
        .next() 
        .unwrap();
    target_area
}

#[derive(Debug, Clone, Copy)]
struct Probe {
    x_pos: i32,
    y_pos: i32,
    x_vel: i32,
    y_vel: i32
}

#[derive(Debug)]
struct TargetArea {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32
}

impl TargetArea {
    fn contains(&self, probe: &Probe) -> bool {
        probe.x_pos >= self.min_x
        && probe.x_pos <= self.max_x
        && probe.y_pos >= self.min_y
        && probe.y_pos <= self.max_y
    }
}