#[macro_use]
mod help;
fn main() {
    let data = input_to_str_vec!("2");

    let mut total: i32 = 0;
    for line in &data {
        if *line == "" {
            continue;
        }
        let result =
            (line.as_bytes()[2] as i8 - line.as_bytes()[0] as i8 - ('X' as i8 - 'A' as i8) + 1)
                .rem_euclid(3);
        let decision = line.as_bytes()[2] as i8 - 'X' as i8;
        let score = (decision + 1 + result * 3) as i32;
        total += score;
    }
    part1!(total);

    total = 0;
    for line in &data {
        if *line == "" {
            continue;
        }
        let result = (line.as_bytes()[2] as i8 - 'X' as i8);
        let decision =
            ((line.as_bytes()[2] as i8 - 'X' as i8) + (line.as_bytes()[0] as i8 - 'A' as i8) - 1)
                .rem_euclid(3);
        let score = (decision + 1 + result * 3) as i32;
        total += score;
    }
    part2!(total);
}
