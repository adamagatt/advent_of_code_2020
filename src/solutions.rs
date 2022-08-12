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
mod solution12;
mod solution13;
mod solution14;
mod solution15;
mod solution16;
mod solution17;
mod solution18;

pub const MAX_SOLUTION: i32 = 18;

pub fn run(choice: i32) {
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
        12 => solution12::solution12(),
        13 => solution13::solution13(),
        14 => solution14::solution14(),
        15 => solution15::solution15(),
        16 => solution16::solution16(),
        17 => solution17::solution17(),
        17 => solution18::solution18(),
        _ => ()
    }
}