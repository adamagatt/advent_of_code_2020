use crate::utils::read_number_lines;

pub fn solution1() -> (){
    let depths = read_number_lines("src/data/solution1.txt");
    println!("{}", solution1a(&depths));
    println!("{}", solution1b(&depths));
}

fn solution1a(depths: &[i32]) -> i32 {
    *(&depths[1..].iter() // Zip slice [1..] with slice starting from beginning
        .zip(depths.iter())
        .filter(|(a, b)| **a > **b)
        .count()) as i32
}

/**
 * The second solution is identical to the first except the index offset for
 * the zipped slices is 3 instead of 1. This is because if we are comparing the
 * sum of window [i0, i1, i2] to the next window [i1, i2, i3], the values i1,i2
 * are present in both and can be cancelled out of both halves of the comparison.
 * The sum of the second window can only exceed the first window if i3 > i0. So
 * in essence we can continue comparing individual values, just with an offset of 3.
 */
fn solution1b(depths: &[i32]) -> i32 {
    *(&depths[3..].iter()
        .zip(depths.iter())
        .filter(|(a, b)| **a > **b)
        .count()) as i32
}