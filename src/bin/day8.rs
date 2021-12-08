#![feature(int_log)]

use std::fmt::Debug;

#[macro_use]
mod help;

type Segments = u8;

#[derive(Default, Clone, Copy)]
struct Digit {
    segments: Segments,
    possible: u16,
}

impl Debug for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut segments = String::new();
        for l in 0..=7 {
            if self.segments & 1 << l > 0 {
                segments.push(('a' as u8 + l) as char)
            }
        }
        let mut possible = vec![];
        for l in 0..=9 {
            if self.possible & 1 << l > 0 {
                possible.push(l)
            }
        }
        f.debug_struct("Digit")
            .field("segments", &segments)
            .field("possible", &possible)
            .finish()
    }
}

macro_rules! possible {
    ($($x:literal),*) => {0 $(| (1<<$x))*};
}

impl Digit {
    fn new(input: &str) -> Digit {
        let mut s = Digit::default();
        let mut seg = 0;
        for l in input.chars() {
            seg |= 1 << (l as u8 - 'a' as u8);
        }
        s.segments = seg;
        s.possible = match input.len() {
            2 => possible!(1),
            3 => possible!(7),
            4 => possible!(4),
            5 => possible!(2, 3, 5),
            6 => possible!(0, 6, 9),
            7 => possible!(8),
            _ => unreachable!(),
        };
        s
    }

    fn is_certain(&self) -> bool {
        self.possible.count_ones() == 1
    }

    fn certain(&self) -> Option<u8> {
        if self.is_certain() {
            Some(self.possible.log2() as u8)
        } else {
            None
        }
    }

    fn mark(&mut self, segments: Segments, digit: u8) {
        let matching = (self.segments & segments).count_ones();
        self.possible &= match (matching, digit) {
            (0, _) => self.possible,

            (1, 0) => possible!(),
            (2, 0) => possible!(1),
            (3, 0) => possible!(4, 7),
            (4, 0) => possible!(2, 3, 5),
            (5, 0) => possible!(6, 9),
            (6, 0) => possible!(8),

            (1, 1) => possible!(2, 5, 6),
            (2, 1) => possible!(0, 3, 4, 7, 8, 9),

            (1, 2) => possible!(1),
            (2, 2) => possible!(7, 4),
            (3, 2) => possible!(5),
            (4, 2) => possible!(0, 3, 6, 9),
            (5, 2) => possible!(8),

            (1, 3) => possible!(),
            (2, 3) => possible!(1),
            (3, 3) => possible!(4, 7),
            (4, 3) => possible!(0, 2, 5, 6),
            (5, 3) => possible!(8, 9),

            (1, 4) => possible!(),
            (2, 4) => possible!(1, 2, 7),
            (3, 4) => possible!(0, 3, 5, 6),
            (4, 4) => possible!(8, 9),

            (1, 5) => possible!(1),
            (2, 5) => possible!(7),
            (3, 5) => possible!(2, 4),
            (4, 5) => possible!(0, 3),
            (5, 5) => possible!(6, 8, 9),

            (1, 6) => possible!(1),
            (2, 6) => possible!(7),
            (3, 6) => possible!(4),
            (4, 6) => possible!(2, 3),
            (5, 6) => possible!(0, 5, 9),
            (6, 6) => possible!(8),

            (1, 7) => possible!(),
            (2, 7) => possible!(1, 2, 4, 5, 6),
            (3, 7) => possible!(0, 3, 8, 9),

            (1, 8) => possible!(),
            (2, 8) => possible!(1),
            (3, 8) => possible!(7),
            (4, 8) => possible!(4),
            (5, 8) => possible!(2, 3, 5),
            (6, 8) => possible!(0, 6, 9),
            (7, 8) => possible!(),

            (1, 9) => possible!(),
            (2, 9) => possible!(1),
            (3, 9) => possible!(7),
            (4, 9) => possible!(2, 4),
            (5, 9) => possible!(0, 3, 5, 6),
            (6, 9) => possible!(8),
            _ => possible!(),
        }
    }
}

fn main() {
    let mut digits = input_to_str_iterator!("8")
        .map(|f| {
            let mut spl = f
                .split(" | ")
                .map(|c| c.split(" ").map(|str| Digit::new(str)));
            let mut first = [Digit::default(); 10];
            for (pos, i) in spl.next().unwrap().enumerate() {
                first[pos] = i;
            }
            let mut second = [Digit::default(); 4];
            for (pos, i) in spl.next().unwrap().enumerate() {
                second[pos] = i;
            }
            (first, second)
        })
        .collect::<Vec<_>>();

    part1!(digits
        .iter()
        .map(|a| a
            .1
            .iter()
            .filter(|b| matches!(b.certain(), Some(1) | Some(4) | Some(7) | Some(8)))
            .count())
        .sum::<usize>());

    part2!(digits
        .iter_mut()
        .map(|a| {
            while a.0.iter().any(|b| !b.is_certain()) {
                if a.0.iter().any(|b| b.possible == 0) {
                    panic!("Failed to solve {:?}", a);
                }
                let certains =
                    a.0.iter()
                        .filter(|c| c.is_certain())
                        .map(|p| *p)
                        .collect::<Vec<_>>();
                for b in a.0.iter_mut().filter(|c| !c.is_certain()) {
                    for certain in &certains {
                        b.mark(certain.segments, certain.certain().unwrap());
                    }
                }
            }
            a.1.iter()
                .enumerate()
                .map(|(pos, b)| {
                    a.0.iter()
                        .find(|c| c.segments == b.segments)
                        .unwrap()
                        .certain()
                        .unwrap() as u32
                        * 10u32.pow(3 - (pos as u32))
                })
                .sum::<u32>()
        })
        .sum::<u32>())
}
