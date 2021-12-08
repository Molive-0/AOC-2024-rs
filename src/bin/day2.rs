#[macro_use]
mod help;
fn main() {
    let data = input_to_str_vec!("2");

    let mut x = 0i32;
    let mut y = 0i32;
    for d in &data {
        match &d[..d.len() - 2] {
            "forward" => x += d[8..].parse::<i32>().unwrap(),
            "down" => y += d[5..].parse::<i32>().unwrap(),
            "up" => y -= d[3..].parse::<i32>().unwrap(),
            _ => unreachable!(),
        }
    }
    part1!(x * y);

    let mut x = 0i32;
    let mut y = 0i32;
    let mut aim = 0i32;
    for d in data {
        match &d[..d.len() - 2] {
            "forward" => {
                let z = d[8..].parse::<i32>().unwrap();
                x += z;
                y += aim * z
            }
            "down" => aim += d[5..].parse::<i32>().unwrap(),
            "up" => aim -= d[3..].parse::<i32>().unwrap(),
            _ => unreachable!(),
        }
    }
    part2!(x * y);
}
