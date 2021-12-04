mod solution1;
mod solution2;
mod solution3;
mod solution4;

pub fn run(choice: i32) -> () {
    match choice {
        1 => solution1::solution1(),
        2 => solution2::solution2(),
        3 => solution3::solution3(),
        4 => solution4::solution4(),
        _ => ()
    }
}