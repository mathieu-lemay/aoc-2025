use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::num::ParseIntError;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};
use itertools::Itertools;
use tracing::{debug, info, warn};

fn main() {
    tracing_init();

    let input = get_input("day08.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point3<T>
where
    T: Clone + Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3<T>
where
    T: Clone + Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl Point3<i64> {
    pub fn distance_to(&self, other: &Self) -> f64 {
        let n = (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2);
        (n as f64).sqrt()
    }
}

impl<T> Display for Point3<T>
where
    T: Display + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{},{},{}", self.x, self.y, self.z))
    }
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let boxes = parse(input).expect("Invalid input");
    let p1 = get_circuits(&boxes, 1000);
    let p2 = 0;

    (p1, p2)
}

#[derive(Debug)]
struct JunctionBox {
    id: usize,
    position: Point3<i64>,
}

#[derive(Debug)]
struct Distance {
    a: usize,
    b: usize,
    distance: f64,
}

#[tracing::instrument(skip_all)]
fn parse(input: &[String]) -> Result<Vec<JunctionBox>, String> {
    input
        .iter()
        .enumerate()
        .map(|(idx, l)| {
            let values: Result<Vec<i64>, ParseIntError> =
                l.split(',').map(|v| v.parse::<i64>()).collect();

            match values {
                Ok(vals) => {
                    if vals.len() == 3 {
                        let p = Point3::new(vals[0], vals[1], vals[2]);
                        Ok(JunctionBox {
                            id: idx,
                            position: p,
                        })
                    } else {
                        Err(format!("Invalid number of values: {}", vals.len()))
                    }
                }
                Err(e) => Err(format!("Invalid entry: {:?}", e)),
            }
        })
        .collect()
}

#[tracing::instrument(skip_all)]
fn get_distances(boxes: &[JunctionBox]) -> Vec<Distance> {
    let mut distances = Vec::with_capacity(boxes.len() * boxes.len() / 2);

    for (i, a) in boxes.iter().enumerate() {
        for b in boxes.iter().skip(i + 1) {
            let d = a.position.distance_to(&b.position);
            distances.push(Distance {
                a: a.id,
                b: b.id,
                distance: d,
            });
        }
    }

    distances
}

#[tracing::instrument(skip_all)]
fn get_circuits(boxes: &[JunctionBox], pairs: usize) -> usize {
    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    let boxmap: HashMap<usize, Point3<i64>> = boxes.iter().map(|b| (b.id, b.position)).collect();

    let mut distances = get_distances(boxes);
    distances.sort_by(|a, b| {
        if a.distance > b.distance {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    for d in distances.iter().take(pairs) {
        debug!(
            "Checking for {}({}) and {}({})",
            d.a, boxmap[&d.a], d.b, boxmap[&d.b]
        );

        let circuit = circuits
            .iter_mut()
            .find(|c| c.contains(&d.a) || c.contains(&d.b));

        match circuit {
            Some(c) => {
                c.insert(d.a);
                c.insert(d.b);
            }
            None => {
                let mut c = HashSet::new();
                c.insert(d.a);
                c.insert(d.b);
                debug!("  Adding them to new circuit");

                circuits.push(c);
            }
        }
    }

    circuits.sort_by(|c1, c2| {
        if c1.len() > c2.len() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    for c in circuits.iter().take(10) {
        info!("Circuit of {} items: {:?}", c.len(), c);
    }

    circuits.iter().map(|c| c.len()).take(3).product()
}

#[cfg(test)]
mod tests {
    use aoc_common::parse_test_input;
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn test_input() -> Vec<String> {
        parse_test_input(
            "
            162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689
            ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day08.txt")
    }

    #[rstest]
    #[test_log::test]
    fn test_p1(test_input: Vec<String>) {
        let boxes = parse(&test_input).expect("Invalid input");
        let res = get_circuits(&boxes, 10);

        assert_eq!(res, 40);
    }

    #[rstest]
    #[test_log::test]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        // let boxes = parse(&puzzle_input).expect("Invalid input");
        // let res = get_circuits(&boxes, 1000);
        //
        // assert_eq!(res, 4896);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let res = 0;

        assert_eq!(res, 1);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let res = 0;

        assert_eq!(res, 1);
    }
}
