use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

pub fn read_string_lines(path: &str) -> Vec<String> {
    iterate_file_lines(path)
        .collect::<Result<Vec<String>, Box<dyn Error>>>()
        .expect("Error parsing data as list of Strings")
}

pub fn read_number_lines(path: &str) -> Vec<i32> {
    iterate_file_lines(path)
        .map(parse_as_int)
        .collect::<Result<Vec<i32>, Box<dyn Error>>>()
        .expect("Error parsing data as list of i32s")
}

pub fn read_lines_by_words(path: &str) -> Vec<Vec<String>> {
    iterate_file_lines(path)
        .map(parse_as_word_list)
        .collect::<Result<Vec<Vec<String>>, Box<dyn Error>>>()
        .expect("Error parsing data as list of word lists")
}

pub fn read_string_int_tuples(path: &str) -> Vec<(String, i32)> {
    iterate_file_lines(path)
        .map(parse_as_string_int_tuple)
        .collect::<Result<Vec<(String, i32)>, Box<dyn Error>>>()
        .expect("Error parsing data as list of (String, i32)s")
}

pub fn read_int_line(path: &str, separator: char) -> Vec<i32> {
    iterate_file_lines(path)
        .next().unwrap().unwrap()
        .split(separator)
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

pub fn read_2d_int_array(path: &str) -> Vec<Vec<u8>> {
    iterate_file_lines(path)
        .map(|line| line.unwrap().chars()
            .map(|digit| digit.to_digit(10).expect("Non-digit character found") as u8)
            .collect::<Vec<u8>>()
        )
        .collect::<Vec<Vec<u8>>>()
}

pub fn read_string_pairs(path: &str, separator: char) -> Vec<(String, String)> {
    iterate_file_lines(path)
        .map(|pair_str| {
            let tokens = pair_str.unwrap()
                .split(separator)
                .map(String::from)
                .collect::<Vec<String>>();
            (tokens[0].clone(), tokens[1].clone())
        })
        .collect::<Vec<(String, String)>>()
}

pub fn iterate_file_lines(path: &str) -> impl Iterator<Item = Result<String, Box<dyn Error>>> {
    let file = File::open(path).unwrap_or_else(|_| panic!("Unable to open file at {}", path));
    BufReader::new(file)
        .lines()
        .map(|line| line.map_err(box_error))
}

fn parse_as_int(string_result: Result<String, Box<dyn Error>>) -> Result<i32, Box<dyn Error>> {
    string_result.and_then(
        |str_value| str_value
            .parse::<i32>()
            .map_err(box_error)
    )
}

fn parse_as_word_list(string_result: Result<String, Box<dyn Error>>) -> Result<Vec<String>, Box<dyn Error>> {
    string_result.map(
        |str_value| str_value
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>()
    )
}

fn parse_as_string_int_tuple(string_result: Result<String, Box<dyn Error>>) -> Result<(String, i32), Box<dyn Error>> {
    string_result.and_then(
        |str_value| {
            let tokens: Vec<&str> = str_value.split_whitespace().collect();
            match tokens[1].parse::<i32>() {
                Ok(parsed) => Ok((String::from(tokens[0]), parsed)),
                Err(err) => Err(box_error(err))
            }
        }
    )
}

fn box_error<E>(e: E) -> Box<dyn Error>
where E: Error + 'static
{
    Box::new(e)
}