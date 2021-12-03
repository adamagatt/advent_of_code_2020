mod solution1;
mod solution2;
mod solution3;

pub fn run(choice: i32) -> () {
    match choice {
        1 => solution1::solution1(),
        2 => solution2::solution2(),
        3 => solution3::solution3(),
        _ => ()
    }
}