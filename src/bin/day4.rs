use std::iter::Sum;

#[derive(Debug)]
enum Marked<A> {
    Unmarked(A),
    Marked(A),
}

impl<A: Default> Default for Marked<A> {
    fn default() -> Self {
        Marked::Unmarked(A::default())
    }
}

#[derive(Debug, Default)]
struct Block<A>([[Marked<A>; 5]; 5]);

impl<A: Sum<A> + Copy + Eq> Block<A> {
    fn check_line(&self) -> bool {
        for y in 0..5 {
            if self.0[y].iter().all(|f| matches!(f, Marked::Marked(_))) {
                return true;
            }
        }
        for y in 0..5 {
            let mut b = true;
            for x in 0..5 {
                b &= matches!(self.0[x][y], Marked::Marked(_))
            }
            if b {
                return b;
            }
        }
        false
    }
    fn _check_block(&self) -> bool {
        let mut b = true;
        for y in 0..5 {
            b &= self.0[y].iter().all(|f| matches!(f, Marked::Marked(_)))
        }
        b
    }
    fn mark(&mut self, check: A) {
        for y in 0..5 {
            for x in 0..5 {
                match self.0[x][y] {
                    Marked::Unmarked(ccheck) => {
                        if ccheck == check {
                            self.0[x][y] = Marked::Marked(check);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    fn get_unmarked(&self) -> A {
        self.0
            .iter()
            .map(|i| {
                i.iter()
                    .filter_map(|f| match f {
                        Marked::Unmarked(e) => Some(*e),
                        Marked::Marked(_) => None,
                    })
                    .sum()
            })
            .sum()
    }
}

#[macro_use]
mod help;
fn main() {
    let mut data = input_to_str_iterator!("4");

    let calls = data
        .next()
        .unwrap()
        .split(",")
        .map(|f| f.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    data.next().unwrap();
    let mut blocks = vec![];
    loop {
        let mut line = data.next();
        if line.is_none() {
            break;
        }
        let mut block: Block<u32> = Block::default();
        for x in 0..5 {
            let mut split = line.unwrap().split(" ");
            for y in 0..5 {
                let mut next = split.next().unwrap();
                if next == "" {
                    next = split.next().unwrap();
                }
                block.0[x][y] = Marked::Unmarked(next.parse::<u32>().unwrap());
            }
            line = data.next();
        }
        blocks.push(block);
    }

    'part1: for call in &calls {
        for b in blocks.iter_mut() {
            b.mark(*call);
            if b.check_line() {
                part1!(b.get_unmarked() * call);
                break 'part1;
            }
        }
    }

    let mut fin = 0;
    'part2: for call in &calls {
        for b in blocks.iter_mut() {
            b.mark(*call);
        }
        let count = blocks.iter().filter(|f| !f.check_line()).count();
        if count == 1 {
            fin = blocks.iter().position(|f| !f.check_line()).unwrap();
        }
        if count == 0 {
            part2!(blocks[fin].get_unmarked() * call);
            break 'part2;
        }
    }
}
