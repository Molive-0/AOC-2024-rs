#[macro_use]
mod help;
fn main() {
    let data = input_to_2d_i32_vec!("1");

    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    for x in data.iter() {
        let value = x.iter().sum::<i32>();
        if value >= 65536 {
            println!("{:?}", x);
        }
        if value > first {
            third = second;
            second = first;
            first = value;
            continue;
        }
        if value > second {
            third = second;
            second = value;
            continue;
        }
        if value > third {
            third = value;
            continue;
        }
    }
    part1!(first);
    part2!(first + second + third);
}
