use std::collections::HashMap;

#[macro_use]
mod help;
fn main() {
    let heightmap = input_to_str_iterator!("9")
        .map(|line| {
            line.chars()
                .map(|ch| ch as u8 - '0' as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut basins: HashMap<(usize, usize), u32> = HashMap::new();

    for (y, line) in heightmap.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if height == &9 {
                continue;
            }
            let mut point = (x, y);
            loop {
                let low_check = is_low(&heightmap, point);
                if low_check.is_none() {
                    continue;
                }
                let low = low_check.unwrap();
                if low == point {
                    basins.insert(
                        point,
                        basins
                            .get(&point)
                            .and_then(|f| Some(*f))
                            .unwrap_or_default()
                            + 1,
                    );
                    break;
                }
                point = low;
            }
        }
    }

    part1!(basins
        .keys()
        .map(|f| (heightmap[f.1][f.0] + 1) as u32)
        .sum::<u32>());

    part2!({
        let mut values = basins.values().map(|f| *f).collect::<Vec<_>>();
        values.sort();
        values.iter().rev().take(3).product::<u32>()
    });
}

fn is_low(heightmap: &Vec<Vec<u8>>, point: (usize, usize)) -> Option<(usize, usize)> {
    let y = point.1;
    let x = point.0;
    let height = heightmap.get(y).and_then(|f| f.get(x)).unwrap();
    if height == &9 {
        return None;
    }
    if let Some(adjacent) = heightmap.get(y.wrapping_sub(1)).and_then(|f| f.get(x)) {
        if adjacent < height {
            return Some((x, y - 1));
        }
    }
    if let Some(adjacent) = heightmap.get(y + 1).and_then(|f| f.get(x)) {
        if adjacent < height {
            return Some((x, y + 1));
        }
    }
    if let Some(adjacent) = heightmap.get(y).and_then(|f| f.get(x + 1)) {
        if adjacent < height {
            return Some((x + 1, y));
        }
    }
    if let Some(adjacent) = heightmap.get(y).and_then(|f| f.get(x.wrapping_sub(1))) {
        if adjacent < height {
            return Some((x - 1, y));
        }
    }
    Some((x, y))
}
