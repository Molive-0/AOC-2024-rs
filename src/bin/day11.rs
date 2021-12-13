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

    for i in 100.. {
        advance(&mut octopi);
        if octopi.iter().flatten().all(|f| f == &0) {
            part2!(i + 1);
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

fn flash(octopi: &mut [[u8; 10]; 10], pos: (usize, usize)) {
    if octopi[pos.1][pos.0] == 0 {
        return;
    }
    octopi[pos.1][pos.0] += 1;
    if octopi[pos.1][pos.0] <= 9 {
        return;
    }

    octopi[pos.1][pos.0] = 0;

    let mut xpos = vec![pos.0];
    let mut ypos = vec![pos.1];
    if pos.0 > 0 {
        xpos.push(pos.0 - 1);
    }
    if pos.0 < 9 {
        xpos.push(pos.0 + 1);
    }
    if pos.1 > 0 {
        ypos.push(pos.1 - 1);
    }
    if pos.1 < 9 {
        ypos.push(pos.1 + 1);
    }
    for x in &xpos {
        for y in &ypos {
            if (*x, *y) == pos {
                continue;
            }
            flash(octopi, (*x, *y));
        }
    }
}
