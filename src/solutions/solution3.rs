use crate::utils::read_string_lines;
use std::collections::HashSet;

pub fn solution3() -> () {
    let readings = read_string_lines("src/data/solution3.txt");
    println!("{}", solution3a(&readings));
    println!("{}", solution3b(&readings));
}

// Number of characters expected for each line of input
const LENGTH: usize = 12;
enum FilterStrategy {
    MostCommon, // Filter to the most common character, with '1' for tiebreaks
    LeastCommon // Filter to the least common character, with '0' for tiebreaks
}

fn solution3a(readings: &[String]) -> u32 {
    let num_readings = readings.len() as i32;
    let gamma_str = readings.iter()
        // We want to build up one counter for each character position (over each reading)
        .fold([0; LENGTH], update_counters_with_reading)
        .iter()
        // Any counter above half the number of lines will map to a '1' character
        .map(|counter| if counter > &(num_readings / 2) {'1'} else {'0'})
        .collect::<String>();
    
    // Can safely unwrap as we know gamma_str consists only of '0' and '1' and fits in u32
    let gamma = u32::from_str_radix(gamma_str.as_str(), 2).unwrap();
    gamma * (!gamma & 0xFFF) // Multiply by inverted bitstring (restricted to bottom 12 bits)
}

fn update_counters_with_reading(mut counters: [i32; LENGTH], reading: &String) -> [i32; LENGTH] {
    for idx in 1..LENGTH {
        // unwrap() character access as we assume each reading is at
        // least LENGTH characters long
        if reading.chars().nth(idx).unwrap() == '1' {
            counters[idx] += 1
        }
    }
    counters
}

fn solution3b(readings: &[String]) -> u32 {
    let oxygen_candidates = readings.iter().collect::<HashSet<&String>>();
    let co2_candidates = oxygen_candidates.clone();

    let oxygen_code = filter_algorithm(oxygen_candidates, FilterStrategy::MostCommon);
    let co2_code = filter_algorithm(co2_candidates, FilterStrategy::LeastCommon);

    u32::from_str_radix(oxygen_code.as_str(), 2).unwrap()
        * u32::from_str_radix(co2_code.as_str(), 2).unwrap()
}

fn filter_algorithm<'a>(mut candidates: HashSet<&'a String>, filter_strategy: FilterStrategy) -> &'a String {
    // One round of elimination for each character, with potential for early exit
    for pos in 0..LENGTH {
        let mut filter_value = most_common_at_position(&candidates, pos);

        // "Least common" strategy uses opposite filter character, even on tie-breaks
        if matches!(filter_strategy, FilterStrategy::LeastCommon) {
            filter_value = if filter_value == '1' { '0' } else { '1' };
        }

        candidates.retain(|candidate|
            candidate.chars().nth(pos).unwrap() == filter_value
        );

        // Early exit if only one candidate remains
        if candidates.len() == 1 {
            break;
        }
    }

    // At this stage we expect only a single candidate to remain in the set
    assert!(candidates.len() == 1, "We don't have a single candidate left!");
    candidates.iter().next().unwrap()
}

fn most_common_at_position(candidates: &HashSet<&String>, pos: usize) -> char {
    let threshold = candidates.len() as f32 / 2.0;
    let ones_count = candidates.iter()
        .filter(|candidate| candidate.chars().nth(pos).unwrap() == '1')
        .count() as f32;
    if ones_count >= threshold {
        '1'
    } else {
        '0'
    }
}