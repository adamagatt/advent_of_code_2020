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

pub fn read_string_int_tuples(path: &str) -> Vec<(String, i32)> {
    iterate_file_lines(path)
        .map(parse_as_string_int_tuple)
        .collect::<Result<Vec<(String, i32)>, Box<dyn Error>>>()
        .expect("Error parsing data as list of (String, i32)s")
}

fn iterate_file_lines(path: &str) -> impl Iterator<Item = Result<String, Box<dyn Error>>> {
    let file = File::open(path).expect(&format!("Unable to open file at {}", path));
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