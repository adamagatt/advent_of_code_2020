use crate::utils::read_lines_by_words;

use std::collections::HashSet;

type Segments = HashSet<char>;
type ProblemLine = (Vec<Segments>, Vec<Segments>);

const UNIQUE_LENGTHS_FOR_DIGIT: [(usize, usize); 4] = [(1, 2), (4, 4), (7, 3), (8, 7)];

pub fn solution8() {
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
        .map(|(inputs, outputs)| calc_output_values(determine_segment_map(inputs), outputs))
        .sum()
}

fn calc_output_values(segment_map: [Option<Segments>; 10], outputs: &[Segments]) -> i32 {
    outputs.iter()
        .map(|output| segment_map.iter().position(|segment_set| segment_set.as_ref().unwrap() == output))
        // Reverse order of found digits so that the increasing enumeration can be used as the
        // exponent in the math of raising that digit to the correct power of 10
        .rev()
        .enumerate()
        .map(|(exponent, digit)| 10i32.pow(exponent as u32) * digit.unwrap() as i32)
        .sum()
}

fn determine_segment_map(inputs: &mut Vec<Segments>) -> [Option<Segments>; 10] {
    let mut segment_map: [Option::<Segments>; 10] = Default::default();
    // 1, 4, 7 and 8 have unique number of characters
    for (code_idx, target_length) in UNIQUE_LENGTHS_FOR_DIGIT {
        segment_map[code_idx] = pop_segments_if(inputs, |unmatched_segment_set|
            unmatched_segment_set.len() == target_length
        );
    }

    // 6 is the only 6-length set where the intersection with 7 is 2 chars
    segment_map[6] = pop_segments_if(inputs, |unmatched_segment_set|
        length_and_overlap_pred(unmatched_segment_set, 6, &segment_map[7], 2)
    );

    // 9 is the only 6-length set where the intersection with 4 is 4 chars
    segment_map[9] = pop_segments_if(inputs, |unmatched_segment_set|
        length_and_overlap_pred(unmatched_segment_set, 6, &segment_map[4], 4)
    );

    // 0 is the last remaining 6-length
    segment_map[0] = pop_segments_if(inputs, |unmatched_segment_set| unmatched_segment_set.len() == 6);

    // 3 is the only 5-length set where the intersection with 7 is 3 chars
    segment_map[3] = pop_segments_if(inputs, |unmatched_segment_set|
        length_and_overlap_pred(unmatched_segment_set, 5, &segment_map[7], 3)
    );

    // 5 is the only 5-length set where the intersection with 6 is 5 chars
    segment_map[5] = pop_segments_if(inputs, |unmatched_segment_set|
        length_and_overlap_pred(unmatched_segment_set, 5, &segment_map[6], 5)
    );

    // 2 is the last remaining set
    segment_map[2] = Some(inputs.remove(0));

    segment_map
}

fn length_and_overlap_pred(segments: &Segments, length: usize, overlap_set: &Option<Segments>, overlap_length: usize) -> bool {
    segments.len() == length
    && segments.intersection(overlap_set.as_ref().unwrap()).count() == overlap_length
}

fn pop_segments_if<F>(segmentss: &mut Vec<Segments>, condition: F) -> Option<Segments>
where F: FnMut(&Segments) -> bool {
    Some(segmentss.remove(segmentss.iter()
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

fn as_char_sets(words: &[String]) -> Vec<Segments> {
    words.iter()
        .map(|word| word.chars()
            .collect::<Segments>()
        )
        .collect::<Vec<Segments>>()
}