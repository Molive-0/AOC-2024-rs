#[macro_use]
mod help;
fn main() {
    let crabs = input_to_i32_comma_vec!("7");
    let lowest: i32 = (crabs.iter().min().unwrap().clone()..=crabs.iter().max().unwrap().clone())
        .map(|pos| crabs.iter().map(|crab| (crab - pos).abs()).sum())
        .min()
        .unwrap();

    part1!(lowest);

    let lowest: i32 = (crabs.iter().min().unwrap().clone()..=crabs.iter().max().unwrap().clone())
        .map(|pos| {
            crabs
                .iter()
                .map(|crab| (((crab - pos).abs() * ((crab - pos).abs() + 1)) / 2))
                .sum()
        })
        .min()
        .unwrap();

    part2!(lowest);
}
