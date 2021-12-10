use crate::utils::read_string_lines;

use std::collections::HashMap;

pub fn solution10() -> () {
    // Map end brackets to their required start bracket, and score if corrupted
    let end_bracket_match = [
        (')', ('(', 3)),
        (']', ('[', 57)),
        ('}', ('{', 1197)),
        ('>', ('<', 25137))
    ].iter().cloned().collect::<HashMap<char, (char, i32)>>();

    let code_lines = read_string_lines("src/data/solution10.txt");
    println!("{}", solution10a(&code_lines, &end_bracket_match));
    println!("{}", solution10b(&code_lines, &end_bracket_match));
}

fn solution10a(code_lines: &Vec<String>, end_bracket_match: &HashMap<char, (char, i32)>) -> i32 {
    code_lines.iter()
        // Return the score of corrupted lists, and filter out those not corrupted
        .filter_map(|code| score_if_corrupted(code, &end_bracket_match))
        .sum()
}

fn score_if_corrupted(code: &String, end_bracket_match: &HashMap<char, (char, i32)>) -> Option<i32> {
    // Bracket matching is a classic stack walking problem
    let mut stack = Vec::<char>::new();

    for cur_char in code.chars() {
        match cur_char {
            // Open brackets are pushed on to stack
            '(' | '{' | '<' | '[' => stack.push(cur_char),
            // Close brackets remove their counterpart from stack, or exit with value if corrupt
            ')' | '}' | '>' | ']' => if let Some(stack_top_bracket) = stack.pop() {
                let (required_start_bracket, corrupt_value) = end_bracket_match[&cur_char]; 
                if stack_top_bracket != required_start_bracket {
                    return Some(corrupt_value);
                }           
            },
            _ => ()
        }
    }

    return None;
}

fn solution10b(code_lines: &Vec<String>, end_bracket_match: &HashMap<char, (char, i32)>) -> u64 {
    // Scores are 64-bit as the scoring math that occurs involves lots of multiplication, and so
    // the total score is exponential to the length of the incomplete string
    let matching_bracket_scores = [
        ('(', 1u64),
        ('[', 2u64),
        ('{', 3u64),
        ('<', 4u64)
    ].iter().cloned().collect::<HashMap<char, u64>>();

    let mut code_scores = code_lines.iter()
        // Return the incomplete bracket lists as remaining open brackets, and filter out
        // those that are corrupted or complete
        .filter_map(|code| remainder_for_incomplete_codes(code, end_bracket_match))
        .filter_map(|code| score_for_code(code, &matching_bracket_scores))
        .collect::<Vec<u64>>();
    
    // Output is median score
    code_scores.sort();
    code_scores[code_scores.len() / 2]
}

fn remainder_for_incomplete_codes(code: &String, end_bracket_match: &HashMap<char, (char, i32)>) -> Option<Vec<char>> {
    // Bracket matching is a classic stack walking problem
    let mut stack = Vec::<char>::new();

    for cur_char in code.chars() {
        match cur_char {
            // Open brackets are pushed on to stack
            '(' | '{' | '<' | '[' => stack.push(cur_char),
            // Close brackets remove their counterpart from stack, or early exit if corrupt
            ')' | '}' | '>' | ']' => if let Some(stack_top_bracket) = stack.pop() {
                let (required_start_bracket, _) = end_bracket_match[&cur_char];
                if stack_top_bracket != required_start_bracket {
                    return None;
                }
            },
            _ => ()
        }
    }

    // At this point our code isn't corrupt but may be complete, in which case we
    // also need to return None so it can be filtered out by the scoring system 
    if stack.len() == 0 {
        None
    } else {
        Some(stack)
    }
}

fn score_for_code(code: Vec<char>, matching_bracket_scores: &HashMap<char, u64>) -> Option<u64> {
    code.iter()
        // To match the open brackets of a code we reverse the sequence and convert to points
        .rev()
        .map(|cur_char| matching_bracket_scores[cur_char])
        // Actual calculation of special scoring formula
        .reduce(|total, cur_value| total*5 + cur_value)
}