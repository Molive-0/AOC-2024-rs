#[macro_use]
mod help;

enum Colours {
    RED,
    GREEN,
    BLUE,
}
fn main() {
    let data = input_to_str_iterator!("2");

    let games = data
        .enumerate()
        .map(|game| {
            (
                game.0 + 1,
                game.1
                    .split(":")
                    .last()
                    .unwrap()
                    .trim()
                    .split(";")
                    .map(|attempt| {
                        attempt
                            .split(",")
                            .map(|colour_set| {
                                let mut info = colour_set.trim().split(" ").take(2);
                                let count: usize = info.next().unwrap().parse().unwrap();
                                let colour = match info.next().unwrap() {
                                    "red" => Colours::RED,
                                    "green" => Colours::GREEN,
                                    "blue" => Colours::BLUE,
                                    _ => unreachable!(),
                                };
                                (count, colour)
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    part1!(games
        .iter()
        .filter(|game| {
            game.1.iter().all(|attempt| {
                attempt.iter().all(|set| match set.1 {
                    Colours::RED => set.0 <= 12,
                    Colours::GREEN => set.0 <= 13,
                    Colours::BLUE => set.0 <= 14,
                })
            })
        })
        .map(|game| game.0)
        .sum::<usize>());

    part2!(games
        .iter()
        .map(|game| {
            game.1
                .iter()
                .fold(vec![0, 0, 0], |mut last, attempt| {
                    for set in attempt {
                        match set.1 {
                            Colours::RED => last[0] = last[0].max(set.0),
                            Colours::GREEN => last[1] = last[1].max(set.0),
                            Colours::BLUE => last[2] = last[2].max(set.0),
                        }
                    }
                    last
                })
                .iter()
                .product::<usize>()
        })
        .sum::<usize>());
}
