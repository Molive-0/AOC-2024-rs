#[macro_use]
mod help;
fn main() {
    let data = input_to_str_iterator!("1");

    part1!(data
        .map(|line| {
            let chars = line.chars();
            let mut forwards = chars.clone();
            let mut backwards = chars.rev().clone();
            let first = forwards.find(char::is_ascii_digit).unwrap();
            let last = backwards.find(char::is_ascii_digit).unwrap();
            let both = format!("{}{}", first, last);
            let number: usize = both.parse().unwrap();
            number
        })
        .sum::<usize>());

    let data = input_to_str_iterator!("1");

    part2!(data
        .map(|line| {
            let chars = line.chars();
            let mut forwards = chars.clone();
            let mut backwards = chars.clone().rev();
            let mut first = {
                let p = forwards.position(|c| c.is_ascii_digit()).unwrap();
                (p, chars.clone().nth(p).unwrap().to_string())
            };
            let mut last = {
                let p = backwards.position(|c| c.is_ascii_digit()).unwrap();
                (p, chars.clone().nth_back(p).unwrap().to_string())
            };
            for number in [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ]
            .iter()
            .enumerate()
            {
                if let Some(n) = line.match_indices(number.1).next() {
                    if n.0 < first.0 {
                        first = (n.0, number.0.to_string());
                    }
                }
                if let Some(n) = line.match_indices(number.1).last() {
                    if n.0 > last.0 {
                        last = (n.0, number.0.to_string());
                    }
                }
            }
            let both = format!("{}{}", first.1, last.1);
            let number: usize = both.parse().unwrap();
            number
        })
        .sum::<usize>());
}
