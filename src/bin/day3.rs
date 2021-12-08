#[macro_use]
mod help;
fn main() {
    let data = input_to_str_iterator!("3")
        .map(|f| {
            f.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let length = data[0].len();

    let gamma = u32::from_str_radix(
        &data
            .iter()
            .fold(vec![0; length], |a, e| {
                a.iter().zip(e).map(|(a, e)| a + e).collect()
            })
            .iter()
            .map(|f| {
                if *f > (data.len() / 2) as u32 {
                    '1'
                } else {
                    '0'
                }
            })
            .collect::<String>(),
        2,
    )
    .unwrap();

    part1!(gamma * (((1 << length) - 1) - gamma));

    let mut ogr = data.clone();
    let mut pos = 0;
    while ogr.len() > 1 {
        let digit = if ogr.iter().map(|f| f[pos]).sum::<u32>() >= ((ogr.len() + 1) / 2) as u32 {
            1
        } else {
            0
        };

        ogr.retain(|f| f[pos] == digit);
        pos += 1;
    }
    let ogr =
        u32::from_str_radix(&ogr[0].iter().map(u32::to_string).collect::<String>(), 2).unwrap();

    let mut c2s = data.clone();
    let mut pos = 0;
    while c2s.len() > 1 {
        let digit = if c2s.iter().map(|f| f[pos]).sum::<u32>() < ((c2s.len() + 1) / 2) as u32 {
            1
        } else {
            0
        };

        c2s.retain(|f| f[pos] == digit);
        pos += 1;
    }
    let c2s =
        u32::from_str_radix(&c2s[0].iter().map(u32::to_string).collect::<String>(), 2).unwrap();

    part2!(ogr * c2s)
}
