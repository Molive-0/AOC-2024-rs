#[macro_use]
mod help;

enum Corruption {
    Corrupt(u32),
    Fine(u64),
}

impl Corruption {
    fn is_corrupt(&self) -> Option<u32> {
        match self {
            &Corruption::Corrupt(u) => Some(u),
            _ => None,
        }
    }
    fn is_fine(&self) -> Option<u64> {
        match self {
            &Corruption::Fine(u) => Some(u),
            _ => None,
        }
    }
}

fn main() {
    let data = input_to_str_vec!("10");

    part1!(data
        .iter()
        .filter_map(|f| corrupt(f).is_corrupt())
        .sum::<u32>());

    part2!({
        let mut fine = data
            .iter()
            .filter_map(|f| corrupt(f).is_fine())
            .collect::<Vec<_>>();
        fine.sort_unstable();
        fine[fine.len() / 2]
    });
}

fn corrupt(input: &str) -> Corruption {
    let mut stack = vec![];
    for ch in input.chars() {
        match ch {
            '<' => stack.push('>'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            '>' | '}' | ']' | ')' => {
                if let Some(pop) = stack.pop() {
                    if pop != ch {
                        return Corruption::Corrupt(match ch {
                            '>' => 25137,
                            '}' => 1197,
                            ']' => 57,
                            ')' => 3,
                            _ => unreachable!(),
                        });
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    Corruption::Fine(
        stack
            .iter()
            .rev()
            .map(|f| match f {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            })
            .fold(0, |acc, x| acc * 5 + x),
    )
}
