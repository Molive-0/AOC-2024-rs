use itertools::Itertools;

#[macro_use]
mod help;
fn main() {
    let data = input_to_str_iterator!("1").map(|f| {
        let mut maps = f.split_whitespace().map(|v| v.parse::<i32>().unwrap());
        (maps.next().unwrap(), maps.next().unwrap())
    }).collect::<Vec<_>>();

    let data_left = data.iter().map(|f| f.0).sorted().collect::<Vec<_>>();
    let data_right = data.iter().map(|f| f.1).sorted().collect::<Vec<_>>();

    part1!(data_left.iter().copied().zip(data_right.iter().copied())
        .map(|(left, right)| {
            let diff = (left - right).abs();
            diff
        })
        .sum::<i32>());

    part2!(data_left.iter().copied()
        .map(|left| {
            let right = data_right.iter().copied().filter(|v| *v == left).count();
            left * (right as i32)
        })
        .sum::<i32>());
}
