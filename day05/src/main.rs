use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};
use itertools::Itertools;

fn main() {
    tracing_init();

    let input = get_input("day05.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let db = parse(input);

    let p1 = count_fresh_ingredients(&db);
    let p2 = count_possible_valid_ingredients(&db);

    (p1, p2)
}

#[derive(Debug, Clone, Copy)]
struct Range {
    s: u64,
    e: u64,
}

impl Range {
    fn contains(&self, v: u64) -> bool {
        v >= self.s && v <= self.e
    }

    fn overlaps(&self, v: &Range) -> bool {
        !(self.s > v.e || v.s > self.e)
    }

    fn merge(&self, v: &Range) -> Range {
        let (&a, _, _, &d) = [v.s, v.e, self.s, self.e]
            .iter()
            .sorted()
            .collect_tuple()
            .unwrap();

        Range { s: a, e: d }
    }
}

#[derive(Debug)]
struct DB {
    ranges: Vec<Range>,
    values: Vec<u64>,
}

#[tracing::instrument(skip_all)]
fn parse(input: &[String]) -> DB {
    let mut db = DB {
        ranges: Vec::new(),
        values: Vec::new(),
    };

    for i in input {
        if i.is_empty() {
            continue;
        }

        let values: Vec<&str> = i.splitn(2, '-').collect();

        match values.len() {
            1 => {
                let v = values[0].parse().expect("invalid value");
                db.values.push(v);
            }
            2 => {
                let s = values[0].parse().expect("invalid value");
                let e = values[1].parse().expect("invalid value");
                db.ranges.push(Range { s, e })
            }
            _ => panic!("Invalid number of values in line: {}", i),
        }
    }

    db
}

fn count_fresh_ingredients(db: &DB) -> usize {
    db.values
        .iter()
        .filter(|&&v| db.ranges.iter().any(|r| r.contains(v)))
        .count()
}

fn count_possible_valid_ingredients(db: &DB) -> u64 {
    let ranges = remove_overlaps(&db.ranges);

    ranges.iter().map(|r| r.e - r.s + 1).sum()
}

fn remove_overlaps(ranges: &[Range]) -> Vec<Range> {
    let mut results = Vec::with_capacity(ranges.len());
    let mut used = Vec::with_capacity(ranges.len());

    for (idx, r) in ranges.iter().enumerate() {
        if used.contains(&idx) {
            continue;
        }

        let mut nr = *r;

        loop {
            let overlap = ranges
                .iter()
                .enumerate()
                .skip(idx + 1)
                .find(|(oidx, or)| !used.contains(oidx) && nr.overlaps(or));

            match overlap {
                Some((oidx, or)) => {
                    nr = nr.merge(or);
                    used.push(oidx);
                }
                None => {
                    break;
                }
            }
        }

        results.push(nr);
    }

    results
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
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day05.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let db = parse(&test_input);
        let res = count_fresh_ingredients(&db);

        assert_eq!(res, 3);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let db = parse(&puzzle_input);
        let res = count_fresh_ingredients(&db);

        assert_eq!(res, 513);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let db = parse(&test_input);
        let res = count_possible_valid_ingredients(&db);

        assert_eq!(res, 14);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let db = parse(&puzzle_input);
        let res = count_possible_valid_ingredients(&db);

        assert_eq!(res, 339668510830757);
    }
}
