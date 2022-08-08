use crate::utils::read_2d_int_array;

type OctopusGraph = Vec<Vec<u8>>;
type FlashGraph = Vec<Vec<bool>>;

pub fn solution11() {
    let octopi = read_2d_int_array("src/data/solution11.txt");
    println!("{}", solution11a(&octopi));
    println!("{}", solution11b(octopi));
}

fn solution11a(octopi_in: &OctopusGraph) -> usize {
    let mut octopi = octopi_in.to_vec();
    let mut flashes = 0;

    for _ in 0..100 {
        flashes += run_simulation(&mut octopi);
    }
    flashes
}

fn solution11b(mut octopi: OctopusGraph) -> usize {
    let num_octopi = octopi.len() * octopi[0].len();

    // Loop forever, while retaining index (first round is index 1)
    for i in 1.. {
        let new_flashes = run_simulation(&mut octopi);

        if new_flashes == num_octopi {
            return i;
        }
    }
    // Unnecessary, as we will never break from the above loop
    0
}

fn run_simulation(octopi: &mut OctopusGraph) -> usize {
    let mut new_flashes = 0;
    let mut flashed = vec![vec![false; octopi[0].len()]; octopi.len()];
    advanced(octopi);

    while let Some(coords) = ready_to_flash(octopi, &flashed) {
        new_flashes += coords.len();
        for (row, col) in coords {
            flashed[row][col] = true;
            increase_around_flash(octopi, row, col);
        }
    }
    reset_flashed(octopi, flashed);
    
    new_flashes
}

fn advanced(octopi: &mut OctopusGraph) {
    octopi.iter_mut()
        .flat_map(|line| line.iter_mut())
        .for_each(|value| *value += 1);
}

fn ready_to_flash(octopi: &OctopusGraph, flashed_graph: &FlashGraph) -> Option<Vec<(usize, usize)>> {
    let flash_list = octopi.iter()
        .enumerate()
        .flat_map(|(row_idx, row)| row.iter()
            .enumerate()
            // "move" required to force closure to take ownership of row_idx, as the compiler suggests
            // that the closure may outlive the row_idx reference, although it should be executed within
            // the filter_map right here immediately
            .filter_map(move |(col_idx, &octopus)| if !flashed_graph[row_idx][col_idx] && octopus > 9 {
                Some((row_idx, col_idx))
            } else {
                None
            })
        )
        .collect::<Vec<(usize, usize)>>();

    // If there are no flash locations we prefer to return None than an empty Vec
    if !flash_list.is_empty() {
        Some(flash_list)
    } else {
        None
    }
}

fn increase_around_flash(octopi: &mut OctopusGraph, row: usize, col: usize) {
    for neighbour_row in usize::saturating_sub(row, 1)..=(row+1) {
        for neighbour_col in usize::saturating_sub(col, 1)..=(col+1) {
            // Check within bounds (no need to check >0 due to saturating_sub)
            if neighbour_row < octopi.len() && neighbour_col < octopi[0].len()
            // Shouldn't increase own counter (though this might not matter)
            && !(neighbour_row == row && neighbour_col == col) {
                octopi[neighbour_row][neighbour_col] += 1;
            } 
        }
    }
}

fn reset_flashed(octopi: &mut OctopusGraph, flashed_graph: FlashGraph) {
    octopi.iter_mut()
        .zip(flashed_graph.iter())
        // Flatten graph to address memory locations in a linear iteration
        .flat_map(|(octopi_line, flashed_line)| octopi_line.iter_mut()
            .zip(flashed_line.iter())
        )
        .for_each(|(octopus, &flashed)| if flashed {*octopus = 0});
}