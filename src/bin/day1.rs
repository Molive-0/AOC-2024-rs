use std::iter::{once, repeat};

#[macro_use]
mod help;
fn main() {
    let data = input_to_i32_vec!("1");

    let iter1 = data.iter();
    let iter2 = once(&i32::MAX).chain(data.iter());
    let count: i32 = iter1
        .zip(iter2)
        .map(|(a, b)| if a.gt(b) { 1 } else { 0 })
        .sum();
    part1!(count);

    let iter1 = data.iter();
    let iter2 = repeat(&i32::MAX).take(3).chain(data.iter());
    let count: i32 = iter1
        .zip(iter2)
        .map(|(a, b)| if a.gt(b) { 1 } else { 0 })
        .sum();
    part2!(count);
}
