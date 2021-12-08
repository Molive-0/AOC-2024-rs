use std::collections::HashMap;
use std::iter::once;

use geo::algorithm::line_intersection::line_intersection;
use geo::{Coordinate, Line};

#[macro_use]
mod help;

fn main() {
    let data = input_to_str_iterator!("5")
        .map(|f| {
            let sides = f
                .split(" -> ")
                .take(2)
                .map(|point| {
                    let points = point
                        .split(",")
                        .take(2)
                        .map(|num| num.parse().unwrap())
                        .collect::<Vec<f32>>();
                    Coordinate {
                        x: points[0],
                        y: points[1],
                    }
                })
                .collect::<Vec<Coordinate<f32>>>();

            Line::new(sides[0], sides[1])
        })
        .collect::<Vec<Line<f32>>>();

    let aligned = data
        .iter()
        .filter(|line| line.dy().floor() == 0.0 || line.dx().floor() == 0.0)
        .collect::<Vec<&Line<f32>>>();

    let normal = data.iter().collect::<Vec<&Line<f32>>>();

    part1!(find(&aligned));
    part2!(find(&normal));
}

fn find(aligned: &Vec<&Line<f32>>) -> usize {
    let mut found: HashMap<Coordinate<u32>, u32> = HashMap::new();
    for (pos, line1) in aligned.iter().enumerate() {
        for line2 in &aligned[(pos + 1)..] {
            match line_intersection(**line1, **line2) {
                Some(geo::line_intersection::LineIntersection::Collinear { intersection: i }) => {
                    let iter1: Box<dyn Iterator<Item = u32>>;
                    if i.dx() == 0.0 {
                        iter1 = Box::new(once(i.start.x as u32).cycle());
                    } else {
                        if i.start.x > i.end.x {
                            iter1 = Box::new((i.end.x as u32..=i.start.x as u32).rev());
                        } else {
                            iter1 = Box::new(i.start.x as u32..=i.end.x as u32);
                        }
                    }
                    let iter2: Box<dyn Iterator<Item = u32>>;
                    if i.dy() == 0.0 {
                        iter2 = Box::new(once(i.start.y as u32).cycle());
                    } else {
                        if i.start.y > i.end.y {
                            iter2 = Box::new((i.end.y as u32..=i.start.y as u32).rev());
                        } else {
                            iter2 = Box::new(i.start.y as u32..=i.end.y as u32);
                        }
                    }
                    for (x, y) in iter1.zip(iter2) {
                        found.insert(
                            Coordinate { x, y },
                            found
                                .get(&Coordinate { x, y })
                                .and_then(|f| Some(*f))
                                .unwrap_or_default()
                                + 1,
                        );
                    }
                }
                Some(geo::line_intersection::LineIntersection::SinglePoint {
                    intersection,
                    ..
                }) => {
                    if intersection.x.fract() < 0.25 || intersection.x.fract() > 0.75 {
                        found.insert(
                            Coordinate {
                                x: intersection.x.round() as u32,
                                y: intersection.y.round() as u32,
                            },
                            found
                                .get(&Coordinate {
                                    x: intersection.x.round() as u32,
                                    y: intersection.y.round() as u32,
                                })
                                .and_then(|f| Some(*f))
                                .unwrap_or_default()
                                + 1,
                        );
                    }
                }
                _ => {}
            }
        }
    }
    found.values().filter(|v| **v > 0).count()
}
