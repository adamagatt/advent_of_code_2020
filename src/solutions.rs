mod solution1;
mod solution2;

pub fn run(choice: i32) -> () {
    match choice {
        1 => solution1::solution1(),
        2 => solution2::solution2(),
        _ => ()
    }
}