use crate::utils::read_string_int_tuples;

pub fn solution2(){
    let commands = read_string_int_tuples("src/data/solution2.txt");
    println!("{}", solution2a(&commands));
    println!("{}", solution2b(&commands));
}

fn solution2a(commands: &[(String, i32)]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for (direction, amount) in commands {
        match direction.as_str() {
            "forward" => horizontal += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => ()
        }
    }
    horizontal * depth
}

fn solution2b(commands: &[(String, i32)]) -> i32 {
    let mut horizontal = 0;
    let mut aim = 0;
    let mut depth = 0;
    for (direction, amount) in commands {
        match direction.as_str() {
            "forward" => {
                horizontal += amount;
                depth += amount * aim;
            },
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => ()
        }
    }
    horizontal * depth
}