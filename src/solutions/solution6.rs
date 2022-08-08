use crate::utils::read_int_line;

pub fn solution6() {
    let fish_timers = read_int_line("src/data/solution6.txt", ',');
    println!("{}", solution6a(&fish_timers));
    println!("{}", solution6b(&fish_timers));
}

fn solution6a(fish_timers_in: &Vec<i32>) -> usize {
    fish_simulation(fish_timers_in, 80)
}

fn solution6b(fish_timers_in: &Vec<i32>) -> usize {
    fish_simulation(fish_timers_in, 256)
}

fn fish_simulation(fish_timers: &Vec<i32>, days: i32) -> usize {

    // Rather than a huge vector of numbers, what we really want is a map
    // of fish timer -> count. This is easier to work with as all fish at
    // the same timer act identically, and we only need 9 entries for the 9
    // distinct timer values. Since the timer values can be used as a 0-based
    // index, we can just store all of our data in an array.
    let mut count_by_timer = [0; 9];
    for &timer in fish_timers {
        count_by_timer[timer as usize] += 1;
    }

    for _ in 0..days {
        count_by_timer.rotate_left(1);
        // The number of 0-timer fish were wrapped to become 8-timer babies,
        // as desired. But the original fish also need to be re-added to the
        // 6-timer position.
        count_by_timer[6] += count_by_timer[8];
    }
    count_by_timer.iter().sum()
}