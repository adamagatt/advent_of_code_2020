use crate::utils::read_string_lines;
use std::collections::HashSet;

// Number of characters expected for each line of input
const LENGTH: usize = 12;
enum FilterStrategy {
    MostCommon, // Filter to the most common character, with '1' for tiebreaks
    LeastCommon // Filter to the least common character, with '0' for tiebreaks
}

pub fn solution3() -> () {
    let readings = read_string_lines("src/data/solution3.txt");
    println!("{}", solution3a(&readings));
    println!("{}", solution3b(&readings));
}

fn solution3a(readings: &[String]) -> u32 {
    let num_readings = readings.len() as i32;
    let gamma_str = readings.iter()
        // We want to build up counters using each reading
        .fold(
            // One counter for each character position
            [0; LENGTH],
            |acc, reading| {
                let mut new_acc = acc.clone();
                for idx in 1..LENGTH {
                    // unwrap() character access as we assume each reading is at
                    // least LENGTH characters long
                    if reading.chars().nth(idx).unwrap() == '1' {
                        new_acc[idx] += 1
                    }
                }
                new_acc
            }
        )
        .iter()
        // Any counter above half the number of lines will map to a '1' character
        .map(|column_count| if column_count > &(num_readings / 2) {'1'} else {'0'})
        .collect::<String>();
    let gamma = u32::from_str_radix(gamma_str.as_str(), 2).unwrap();
    gamma * (!gamma & 0xFFF) // Multiply by inverted bitstring (restricted to bottom 12 bits)
}

fn solution3b(readings: &[String]) -> u32 {
    let oxygen_candidates = readings.iter().collect::<HashSet<&String>>();
    let co2_candidates = oxygen_candidates.clone();

    let oxygen_code = filter_algorithm(&oxygen_candidates, FilterStrategy::MostCommon);
    let co2_code = filter_algorithm(&co2_candidates, FilterStrategy::LeastCommon);

    u32::from_str_radix(oxygen_code.as_str(), 2).unwrap()
        * u32::from_str_radix(co2_code.as_str(), 2).unwrap()
}

fn filter_algorithm<'a>(candidates_in: &HashSet<&'a String>, filter_strategy: FilterStrategy) -> &'a String {
    // Clone input to have a Set we can in-place filter 
    let mut candidates = candidates_in.clone();

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
    if candidates.len() != 1 {
        panic!("We don't have a single candidate left!")
    }
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