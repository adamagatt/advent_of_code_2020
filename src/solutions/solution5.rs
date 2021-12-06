use crate::utils::read_string_lines;
use std::cmp::{max, min};
use std::str::FromStr;
use itertools::Itertools;

type VentLine = [(usize, usize); 2];

pub fn solution5() -> () {
    let vent_lines = parse_vent_lines(&read_string_lines("src/data/solution5.txt"));
    println!("{}", solution5a(&vent_lines));
    println!("{}", solution5b(&vent_lines));
}

fn solution5a(lines: &[VentLine]) -> usize {
    count_vent_overlap(lines, false)
}

fn solution5b(lines: &[VentLine]) -> usize {
    count_vent_overlap(lines, true)
}

fn count_vent_overlap(lines: &[VentLine], allow_diagonals: bool) -> usize {
    
    let [row_min, col_min, row_max, col_max] = lines.iter()
    .flat_map(|line| line.iter())
    .fold(
        [usize::MAX, usize::MAX, 0, 0],
        |extremes, &point|
        [min(extremes[0], point.0), min(extremes[1], point.1), max(extremes[2], point.0), max(extremes[3], point.1)]
    );

    let cols = (col_max-col_min+1) as usize;
    let rows = (row_max-row_min+1) as usize;

    let mut grid = vec![vec![0; rows]; cols];
    for &[(row1, col1), (row2, col2)] in lines {
        if row1 == row2 {
            for col in min(col1, col2)..(max(col1, col2)+1) {
                grid[col-col_min][row1-row_min] += 1;
            }
        } else if col1 == col2 {
            for row in min(row1, row2)..(max(row1, row2)+1) {
                grid[col1-col_min][row-row_min] += 1;
            }
        // Diagonal case only added to grid for Solution 5b
        } else if allow_diagonals{
            // Do we need to step forward or backward for each axis?
            let col_dir: i32 = if col2 > col1 {1} else {-1};
            let row_dir: i32 = if row2 > row1 {1} else {-1};

            // Can calculate number of steps to take using either axis as
            // it will be the same for a 45-degree aligned line
            let num_steps = (row2 as i32 - row1 as i32).abs() + 1;

            for step_num in 0..num_steps {
                let col_pos = (col1 as i32 + step_num * col_dir) as usize;
                let row_pos = (row1 as i32 + step_num * row_dir) as usize;         
                grid[col_pos - col_min][row_pos - row_min] += 1;
            }
        }
    }

    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&square| square >= 2)
        .count()
}

fn parse_vent_lines(lines: &[String]) -> Vec<VentLine> {
    lines.iter()
        .map(|line| {
            let pairs = line.split("->").map(|pair|
                pair.split(",")
                    .map(|num_str| usize::from_str(num_str.trim()).unwrap())
                    .next_tuple().unwrap()
            ).collect::<Vec<(usize, usize)>>();
            [pairs[0], pairs[1]]
        })
        .collect::<Vec<VentLine>>()
}