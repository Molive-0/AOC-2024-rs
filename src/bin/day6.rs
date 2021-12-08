#![feature(slice_group_by)]

#[macro_use]
mod help;
fn main() {
    let mut fishies = input_to_i32_comma_vec!("6");
    fishies.sort();
    let fishy_groups = fishies
        .group_by(|a, b| a == b)
        .map(|a| (a[0], a.len()))
        .collect::<Vec<_>>();
    let mut fishies = [0u64; 9];
    for (pos, count) in fishy_groups {
        fishies[pos as usize] = count as u64;
    }

    let mut pointer = 0;
    for _ in 0..80 {
        iter_fish(&mut fishies, &mut pointer);
    }
    part1!(fishies.iter().sum::<u64>());
    for _ in 80..256 {
        iter_fish(&mut fishies, &mut pointer);
    }
    part2!(fishies.iter().sum::<u64>());
}

fn iter_fish(fishies: &mut [u64; 9], pointer: &mut usize) {
    let birth_index = *pointer;
    let reset_index = (*pointer + 7) % 9;
    fishies[reset_index] += fishies[birth_index];
    *pointer = if *pointer == 8 { 0 } else { *pointer + 1 }
}
