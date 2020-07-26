#![allow(dead_code)]

mod generate_random;

fn main() {
    let min: i32 = 10;
    let max: i32 = 999;
    generate_random::random_number_in_range(min, max);
}
