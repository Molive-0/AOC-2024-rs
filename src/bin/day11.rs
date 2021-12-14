#[macro_use]
mod help;
fn main() {
    let octopi = input_to_str_iterator!("11")
        .flat_map(|line| {
            let v = line
                .chars()
                .map(|ch| ch as u8 - '0' as u8)
                .collect::<Vec<_>>();
            v.try_into()
        })
        .collect::<Vec<[u8; 10]>>();
    let mut octopi: [[u8; 10]; 10] = octopi.try_into().unwrap();

    let mut flashed = 0;
    for _ in 0..100 {
        advance(&mut octopi);
        flashed += octopi.iter().flatten().filter(|f| f == &&0).count();
    }
    part1!(flashed);

    for i in 101.. {
        advance(&mut octopi);
        if octopi.iter().flatten().all(|f| f == &0) {
            part2!(i);
            break;
        }
    }
}

fn advance(octopi: &mut [[u8; 10]; 10]) {
    for octopus in octopi.iter_mut().flatten() {
        *octopus += 1;
    }
    for x in 0..10 {
        for y in 0..10 {
            if octopi[y][x] > 9 {
                flash(octopi, (x, y));
            }
        }
    }
}

pub fn flash(octopi: &mut [[u8; 10]; 10], pos: (usize, usize)) {
    let octopus;
    unsafe {
        octopus = octopi.get_unchecked_mut(pos.1).get_unchecked_mut(pos.0);
    }
    if *octopus == 0 {
        return;
    }
    *octopus += 1;
    if *octopus <= 9 {
        return;
    }

    *octopus = 0;

    if pos.0 > 0 {
        flash(octopi, (pos.0 - 1, pos.1));
        if pos.1 > 0 {
            flash(octopi, (pos.0 - 1, pos.1 - 1));
        }
        if pos.1 < 9 {
            flash(octopi, (pos.0 - 1, pos.1 + 1));
        }
    }
    if pos.0 < 9 {
        flash(octopi, (pos.0 + 1, pos.1));
        if pos.1 > 0 {
            flash(octopi, (pos.0 + 1, pos.1 - 1));
        }
        if pos.1 < 9 {
            flash(octopi, (pos.0 + 1, pos.1 + 1));
        }
    }
    if pos.1 > 0 {
        flash(octopi, (pos.0, pos.1 - 1));
    }
    if pos.1 < 9 {
        flash(octopi, (pos.0, pos.1 + 1));
    }
}
