use crate::utils::read_lines_by_words;

use std::collections::HashSet;

type CharSet = HashSet<char>;
type ProblemLine = (Vec<CharSet>, Vec<CharSet>);

const UNIQUE_LENGTHS_FOR_DIGIT: [(usize, usize); 4] = [(1, 2), (4, 4), (7, 3), (8, 7)];

pub fn solution8() -> () {
    let code_lines = split_input_output(read_lines_by_words("src/data/solution8.txt"));
    println!("{}", solution8a(code_lines.clone()));
    println!("{}", solution8b(code_lines));
}

fn solution8a(code_lines: Vec<ProblemLine>) -> usize {
    let unique_lengths = UNIQUE_LENGTHS_FOR_DIGIT.iter()
        .map(|(_digit, length)| *length)
        .collect::<Vec<usize>>();

    code_lines.iter()
        .map(|(_input, output)| output.iter()
            .filter(|word| unique_lengths.contains(&word.len()))
            .count()
        )
        .sum()
}

fn solution8b(mut code_lines: Vec<ProblemLine>) -> i32 {
    code_lines.iter_mut()
        .map(|(inputs, outputs)| calc_output_values(determine_charset_map(inputs), outputs))
        .sum()
}

fn calc_output_values(charset_map: [Option<CharSet>; 10], outputs: &Vec<CharSet>) -> i32 {
    outputs.iter()
        .map(|output| charset_map.iter().position(|charset| charset.as_ref().unwrap() == output))
        // Reverse order of found digits so that the increasing enumeration can be used as the
        // exponent in the math of raising that digit to the correct power of 10
        .rev()
        .enumerate()
        .map(|(exponent, digit)| 10i32.pow(exponent as u32) * digit.unwrap() as i32)
        .sum()
}

fn determine_charset_map(inputs: &mut Vec<CharSet>) -> [Option<CharSet>; 10] {
    let mut codes: [Option::<CharSet>; 10] = Default::default();
    // 1, 4, 7 and 8 have unique number of characters
    for (code_idx, target_length) in UNIQUE_LENGTHS_FOR_DIGIT {
        codes[code_idx] = pop_charset_if(inputs, |charset|
            charset.len() == target_length
        );
    }

    // 6 is the only 6-length set where the intersection with 7 is 2 chars
    codes[6] = pop_charset_if(inputs, |charset|
        length_and_overlap_pred(charset, 6, &codes[7], 2)
    );

    // 9 is the only 6-length set where the intersection with 4 is 4 chars
    codes[9] = pop_charset_if(inputs, |charset|
        length_and_overlap_pred(charset, 6, &codes[4], 4)
    );

    // 0 is the last remaining 6-length
    codes[0] = pop_charset_if(inputs, |charset| charset.len() == 6);

    // 3 is the only 5-length set where the intersection with 7 is 3 chars
    codes[3] = pop_charset_if(inputs, |charset|
        length_and_overlap_pred(charset, 5, &codes[7], 3)
    );

    // 5 is the only 5-length set where the intersection with 6 is 5 chars
    codes[5] = pop_charset_if(inputs, |charset|
        length_and_overlap_pred(charset, 5, &codes[6], 5)
    );

    // 2 is the last remaining set
    codes[2] = Some(inputs.remove(0));

    codes
}

fn length_and_overlap_pred(charset: &CharSet, length: usize, overlap_set: &Option<CharSet>, overlap_length: usize) -> bool {
    charset.len() == length
    && charset.intersection(overlap_set.as_ref().unwrap()).count() == overlap_length
}

fn pop_charset_if<F>(charsets: &mut Vec<CharSet>, condition: F) -> Option<CharSet>
where F: FnMut(&CharSet) -> bool {
    Some(charsets.remove(charsets.iter()
        .position(condition)
        .unwrap()
    ))
}

fn split_input_output(words: Vec<Vec<String>>) -> Vec<ProblemLine> {
    words.iter()
        .map(|line| {
            let mut sections = line.split(|word| word == "|");
            (as_char_sets(sections.next().unwrap()), as_char_sets(sections.next().unwrap()))
        })
        .collect::<Vec<ProblemLine>>()
}

fn as_char_sets(words: &[String]) -> Vec<CharSet> {
    words.iter()
        .map(|word| word.chars()
            .collect::<CharSet>()
        )
        .collect::<Vec<CharSet>>()
}