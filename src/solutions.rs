mod solution1;
mod solution2;
mod solution3;
mod solution4;
mod solution5;
mod solution6;
mod solution7;
mod solution8;
mod solution9;
mod solution10;
mod solution11;
mod solution13;

pub const MAX_SOLUTION: i32 = 13;

pub fn run(choice: i32) -> () {
    match choice {
        1 => solution1::solution1(),
        2 => solution2::solution2(),
        3 => solution3::solution3(),
        4 => solution4::solution4(),
        5 => solution5::solution5(),
        6 => solution6::solution6(),
        7 => solution7::solution7(),
        8 => solution8::solution8(),
        9 => solution9::solution9(),
        10 => solution10::solution10(),
        11 => solution11::solution11(),
        13 => solution13::solution13(),
        _ => ()
    }
}