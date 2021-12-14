use crate::utils::read_string_lines;

use std::collections::{HashMap, HashSet};
use std::cmp::{max, min};

pub fn solution14 () {
    let (template, ruleset) = parse_input(read_string_lines("src/data/solution14.txt"));
    println!("{}", solution14a(&template, &ruleset));
    println!("{}", solution14b(&template, &ruleset));
}

type Polymer = (char, char);
type Ruleset = HashMap<Polymer, String>;

#[derive (PartialEq)]
enum ITERATIONS {TEN, FORTY}

fn solution14a(template: &str, ruleset: &Ruleset) -> usize {
    polymer_algorithm(template, ruleset, ITERATIONS::TEN)
}

fn solution14b(template: &str, ruleset: &Ruleset) -> usize {
    polymer_algorithm(template, ruleset, ITERATIONS::FORTY)
}

fn polymer_algorithm(template: &str, ruleset: &Ruleset, iterations: ITERATIONS) -> usize {
    // We can figure out 10 steps of evolution for each pair of characters,
    // at which point we can simply apply the results of this to the original
    // template string pairs to see what the string would look like after
    // the 10 iterations
    let ruleset_after_10 = ruleset.keys()
        .map(|&(char_1, char_2)| ((char_1, char_2), format!("{}{}", char_1, char_2)))
        .map(|(pair, orig_key)| (
            pair,
            (0..10).fold(orig_key,
                |key_evolution, i| {
                    println!("A: {}", i);
                    apply_ruleset(key_evolution, ruleset)
                }
            )
        ))
        // We can remove the first and last character from the iterative ruleset map
        .map(|(pair, iterated_key)| (
            pair,
            iterated_key[1..iterated_key.len()-1].to_string()
        ))
        .collect::<Ruleset>();

    let final_apply_rounds = if iterations == ITERATIONS::FORTY {4} else {1};

    difference_most_least_common(
        (0..final_apply_rounds).fold(String::from(template),
            |key_evolution, i| {
                println!("B: {}", i);
                apply_ruleset(key_evolution, &ruleset_after_10)
            }
        ),
        ruleset.values()
            .map(|polymer_char| polymer_char.chars().nth(0).unwrap())
            .collect::<HashSet<char>>()
    )
}

fn apply_ruleset(string_in: String, ruleset: &Ruleset) -> String {
    let mut string_out = string_in.chars().zip(string_in[1..].chars())
        .map(|(char_1, char_2)| format!("{}{}", char_1, ruleset[&(char_1, char_2)]))
        .collect::<String>();
    
    string_out.push(string_in.chars().last().unwrap());

    string_out
}

fn difference_most_least_common(final_code: String, values: HashSet<char>) -> usize {
    let mut most_count = 0;
    let mut least_count = usize::MAX;

    values.iter()
        .map(|&cur_char|
            final_code.chars()
                .filter(|&code_char| code_char == cur_char)
                .count()
        )
        .for_each(|count| {
            most_count = max(count, most_count);
            least_count = min(count, least_count);
        });
    
    most_count - least_count
}

fn parse_input(lines: Vec<String>) -> (String, Ruleset) {
    let template = lines[0].clone();
    
    let ruleset = lines.iter()
        // Begin reading from 3rd line to get ruleset
        .skip(2)
        .take_while(|line| line.len() > 0)
        // Positions of characters of interest are the same for every line
        .map(|rule| (
            (rule.chars().nth(0).unwrap(), rule.chars().nth(1).unwrap()), 
            rule[6..7].to_string()
        ))
        .collect::<Ruleset>();
    
        (template, ruleset)
}