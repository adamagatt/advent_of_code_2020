use crate::utils::read_int_line;

use std::collections::{BTreeMap, HashMap};
use std::cmp::{min, max};

pub fn solution7() -> () {
    let crab_positions = read_int_line("src/data/solution7.txt", ',');
    println!("{}", solution7a(&crab_positions));
    println!("{}", solution7b(&crab_positions));
}

fn solution7a(crab_positions_in: &[i32]) -> i32 {
    let (position_map, min_pos, max_pos) = make_position_tree(crab_positions_in);

    // Use the average of the extremes as a first good guess
    let mut guess = (max_pos + min_pos) / 2;

    // Lower our guess while there are more crabs lower than this position
    while count_lower_than(&position_map, guess) > count_higher_than(&position_map, guess) {
        guess -= 1;
    }

    // Increase our guess while there are more crabs above this position
    while count_lower_than(&position_map, guess) < count_higher_than(&position_map, guess) {
        guess += 1;
    }
    
    position_map.iter().map(|(position, crabs)| (position-guess).abs() * crabs).sum()
}

fn solution7b(crab_positions_in: &[i32]) -> i32 {
    let (position_map, min_pos, max_pos) = make_position_tree(crab_positions_in);

    // Use the average of the extremes as a first good guess
    let mut guess = (max_pos + min_pos) / 2;
    
    // We want to cache the costs of previous guesses to avoid having to recalculate costly
    // fuel calculations unnecessarily
    let mut cost_cache = HashMap::<i32, i32>::new();

    loop {
        // Sample the fuel costs for our guess position and its neighbours on each side
        let lower_cost = fuel_need_for_position_cached(&position_map, guess-1, &mut cost_cache);
        let guess_cost = fuel_need_for_position_cached(&position_map, guess, &mut cost_cache);
        let higher_cost = fuel_need_for_position_cached(&position_map, guess+1, &mut cost_cache);

        // Creep our guess lower or higher to follow the gradient to the lowest cost. This approach
        // will fail if trapped in a local minimum but I have a feeling the problem will not
        // result in these being possible
        if lower_cost < guess_cost {
            guess -= 1;
        } else if higher_cost < guess_cost {
            guess += 1;
        } else {
            // If our guess has a lower cost than its neighbours then we found the minimum
            break;
        }
    }
    
    fuel_need_for_position_cached(&position_map, guess, &mut cost_cache)
}

fn make_position_tree(positions: &[i32]) -> (BTreeMap<i32, i32>, i32, i32) {
    let mut position_map = BTreeMap::<i32, i32>::new();
    let mut min_pos = i32::MAX;
    let mut max_pos = 0;
    for &position in positions {
        position_map.entry(position)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        
        min_pos = min(min_pos, position);
        max_pos = max(max_pos, position);
    }
    (position_map, min_pos, max_pos)
}

fn count_lower_than(tree: &BTreeMap<i32, i32>, pivot: i32) -> i32 {
    tree.range(..pivot).map(|(_key, value)| value).sum()
}

fn count_higher_than(tree: &BTreeMap<i32, i32>, pivot: i32) -> i32 {
    tree.range((pivot+1)..).map(|(_key, value)| value).sum()
}

fn fuel_need_for_position(tree: &BTreeMap<i32, i32>, position: i32) -> i32 {
    tree.iter().map(|(key, value)| fuel_for_distance((key - position).abs()) * value).sum()
}

// Cache for map/position, as this may be a costly calculation
fn fuel_need_for_position_cached(tree: &BTreeMap<i32, i32>, position: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    *(cache
        .entry(position)
        .or_insert(fuel_need_for_position(tree, position))
    )
}

fn fuel_for_distance(distance: i32) -> i32 {
    // The amount of fuel needed for a given distance follows a quadratic
    // triangular function (i.e. 1=>1, 2=>3, 3=>6, 4=>10 etc)
    distance * (distance + 1) / 2
}