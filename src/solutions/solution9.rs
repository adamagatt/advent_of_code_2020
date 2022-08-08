use crate::utils::read_2d_int_array;

pub fn solution9() {
    let floor_heights = read_2d_int_array("src/data/solution9.txt");
    println!("{}", solution9a(&floor_heights));
    println!("{}", solution9b(&floor_heights));
}

// Cells at ridge height separate basins and act as a blocker to our flood fill search approach
const RIDGE: u8 = 9;

fn solution9a(floor_heights: &Vec<Vec<u8>>) -> u32 {
    let mut risk_sum: u32 = 0;
    for row in 0..floor_heights.len() {
        // Assuming all rows are the same length
        for col in 0..floor_heights[0].len() {
            if lower_than_neighbours(floor_heights, row, col) {
                risk_sum += (floor_heights[row][col] as u32) + 1;
            }
        }
    }
    risk_sum
}

fn lower_than_neighbours(floor_heights: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    let height = floor_heights[row][col];
    (row == 0 || height < floor_heights[row-1][col])
    && (row == (floor_heights.len()-1) || height < floor_heights[row+1][col])
    && (col == 0 || height < floor_heights[row][col-1])
    && (col == (floor_heights[0].len()-1) || height < floor_heights[row][col+1])
}

fn solution9b(floor_heights: &Vec<Vec<u8>>) -> u32 {
    // Boolean "already visited by flood fill" map with same dimensions as height
    // map. Locations at ridge height already count as visited.
    let mut visited = floor_heights.iter()
        .map(|line| line.iter()
            .map(|&height| height == 9)
            .collect::<Vec<bool>>()
        )
        .collect::<Vec<Vec<bool>>>();
    
    let mut basin_sizes = Vec::<u32>::new();

    // Each location should be checked as the potential start of a flood fill
    for row in 0..floor_heights.len() {
        for col in 0..floor_heights[0].len() {
            if !visited[row][col] {
                basin_sizes.push(basin_size_from_flood_fill(floor_heights, &mut visited, row, col));
            }
        }
    }

    // To sort in descending order we sort then reverse
    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

fn basin_size_from_flood_fill(floor_heights: &Vec<Vec<u8>>, visited: &mut [Vec<bool>], row: usize, col: usize) -> u32 {
    // Keep a list of discovered basin locations to continue the flood fill through
    let mut to_visit = vec![(row, col)];
    let mut basin_size = 0;

    // Continue until we run out of coordinates to visit
    while !to_visit.is_empty() {
        // Retrieve the next coordinate from the list and mark as visited
        let (row, col) = to_visit.remove(0);
        basin_size += 1;
        visited[row][col] = true;
        // Determine valid neighbours to visit and add to list
        to_visit.append(&mut get_unvisited_neighbours(floor_heights, visited, &to_visit, row as i32, col as i32));
    }
    basin_size
}

fn get_unvisited_neighbours(floor_heights: &Vec<Vec<u8>>, visited: &[Vec<bool>], to_visit: &[(usize, usize)], row: i32, col: i32) -> Vec<(usize, usize)> {
    // Checking all orthogonal neighbours
    [(row-1, col), (row+1, col), (row, col-1), (row, col+1)].iter()
        .filter(|(neighbour_row, neighbour_col)|
            // Row and column within acceptable ranges
            *neighbour_row >= 0 && *neighbour_row < floor_heights.len() as i32
            && *neighbour_col >= 0 && *neighbour_col < floor_heights[0].len() as i32
            // Candidate neighbour isn't at ridge height
            && floor_heights[*neighbour_row as usize][*neighbour_col as usize] != RIDGE
            // We haven't visited this neighbour yet
            && !visited[*neighbour_row as usize][*neighbour_col as usize]
            // We don't already have this neighbour on our list of coordinates to visit
            && !to_visit.contains(&(*neighbour_row as usize, *neighbour_col as usize))
        )
        .map(|&(row, col)| (row as usize, col as usize))
        .collect()
}