use crate::utils::read_string_lines;

use std::collections::{HashMap, HashSet};
use std::cmp::{max, min};

pub fn solution14 () {
    let (template, children) = parse_input(read_string_lines("src/data/solution14.txt"));
    println!("{}", solution14a(&template, &children));
    println!("{}", solution14b(&template, &children));
}

// A polymer is a pair of characters
type Polymer = (char, char);
// A map of polymers to both expansions it will grow to
type Children = HashMap<Polymer, [Polymer; 2]>;
// A counter of character frequency
type Counter = HashMap<char, usize>;
// A counter for each polymer, created at each iteration based on the previous
type Generation = HashMap<Polymer, Counter>;

fn solution14a(template: &str, children: &Children) -> usize {
    polymer_algorithm(template, children, 10)
}

fn solution14b(template: &str, children: &Children) -> usize {
    polymer_algorithm(template, children, 40)
}

fn polymer_algorithm(template: &str, children: &Children, iterations: usize) -> usize {
    // Create the counters for the character content of each base polymer at the
    // first generation, before any insertion iterations have happened
    let mut generation_counters = vec![create_first_generation(children)];

    // We proceed through each iteration, calculating the histogram of characters that an
    // original polymer would have expanded to by that iteration
    for i in 0..iterations {
        generation_counters.push(progress_generation(&generation_counters[i], children));
    }

    // Apply the last generation map to our input string and perform output calculation
    difference_most_least_common(calculate_final_counter(template, &generation_counters[iterations]))
}

fn create_first_generation(children_rules: &Children) -> Generation {

    // Determine the letter content of each possible polymer at the first
    // generation before any insertion has happened. This will be equal to
    // simply the first character of the polymer, e.g. (a, b) => {a}. We leave
    // out the second character as the next pairwise polymer pair (e.g. (b, c))
    // will include that character in its own histogram.
    children_rules.keys()
        .map(|&polymer| {
            let mut counter_for_polymer = Counter::new();
            counter_for_polymer.insert(polymer.0, 1);
            (
                polymer,
                counter_for_polymer
            )
        })
        .collect::<Generation>()
}

fn progress_generation(old_generation: &Generation, children: &Children) -> Generation {
    // The histogram for a polygon at a generation is derived from the histograms of its
    // two expansion polymers from the previous generation. The "children" map is followed
    // to determine these child polymers and their two histograms are looked up and combined
    // with each other.
    old_generation.iter()
        .map(|(&polymer, _)|
            (
                polymer,
                children.get(&polymer).unwrap().iter()
                    .map(|&child| old_generation.get(&child).unwrap())
                    .cloned() // The reduction function will need full Generations instead of just refs
                    .reduce(join_counters)
                    .unwrap()
            )
        )
        .collect::<Generation>()
}

fn join_counters(counter1: Counter, counter2: Counter) -> Counter {
    // Create the unique set of keys over both counters
    let mut keys = counter1.keys().cloned().collect::<HashSet<char>>();
    for key in counter2.keys() {
        keys.insert(*key);
    }

    let mut counter_out = Counter::new();

    // For each key, add the count (if any) from each counter together
    for key in keys {
        let mut count = 0;
        for counter in [&counter1, &counter2] {
            if counter.contains_key(&key) {
                count += counter.get(&key).unwrap();
            }
        }

        counter_out.insert(key, count);
    }
    counter_out
}

fn calculate_final_counter(template: &str, generation: &Generation) -> Counter {
    // First we look up the last generation counter to each pairwise polymer of
    // the input string, and then combine all those counters together
    // NOTE: array_windows() might be more elegant here but is not supported for chars()
    let mut final_counter = template.chars().zip(template[1..].chars())
        .map(|chars| generation[&chars].clone())
        .reduce(join_counters)
        .unwrap();

    // The last character in the string is absent from all counters and so we
    // add it back in manually after all other processing
    *final_counter
        .get_mut(&template.chars()
            .last()
            .unwrap()
        )
        .unwrap() += 1;

    final_counter
}

fn difference_most_least_common(counter: Counter) -> usize {
    let mut most = 0_usize;
    let mut least = usize::MAX;

    for &value in counter.values() {
        most = max(most, value);
        least = min(least, value);
    }

    most - least
}

fn parse_input(lines: Vec<String>) -> (String, Children) {
    let template = lines[0].clone();
    
    let children = lines.iter()
        // Begin reading from 3rd line to get ruleset
        .skip(2)
        .take_while(|line| !line.is_empty())
        // Positions of characters of interest are the same for every line
        .map(|rule| {
            let left_char = rule.chars().nth(0).unwrap();
            let mid_char = rule.chars().nth(6).unwrap();
            let right_char = rule.chars().nth(1).unwrap();
            (
                (left_char, right_char),
                [(left_char, mid_char), (mid_char, right_char)]
            )
        })
        .collect::<Children>();
    
        (template, children)
}