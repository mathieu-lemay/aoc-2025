use std::fmt::Display;
use std::time::Instant;

use aoc_common::{Point, format_duration, get_input, tracing_init};
use itertools::Itertools;
use tracing::debug;

fn main() {
    tracing_init();

    let input = get_input("day09.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let points = parse(input).expect("invalid input");

    let p1 = get_biggest_rectangle(&points);
    let p2 = "oh hell nah";

    (p1, p2)
}

fn get_rectangle_size(p1: &Point<i64>, p2: &Point<i64>) -> i64 {
    ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1)
}

#[tracing::instrument(skip_all)]
fn parse(input: &[String]) -> Result<Vec<Point<i64>>, String> {
    let mut points = Vec::new();
    for l in input {
        if let Some((x, y)) = l.split(',').collect_tuple() {
            let x = x
                .parse::<i64>()
                .map_err(|e| format!("invalid entry: {}: {}", l, e))?;
            let y = y
                .parse::<i64>()
                .map_err(|e| format!("invalid entry: {}: {}", l, e))?;
            points.push(Point::new(x, y));
        } else {
            return Err(format!("invalid entry: {}", l));
        }
    }

    Ok(points)
}

#[tracing::instrument(skip_all)]
fn get_biggest_rectangle(points: &Vec<Point<i64>>) -> i64 {
    let mut max = 0;

    for (i, a) in points.iter().enumerate() {
        for b in points.iter().skip(i + 1) {
            let size = get_rectangle_size(a, b);
            if size > max {
                debug!("Found bigger rectangle with {} and {}: {}", a, b, size);
                max = size;
            }
        }
    }

    max
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
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day09.txt")
    }

    #[rstest]
    #[test_log::test]
    fn test_p1(test_input: Vec<String>) {
        let grid = parse(&test_input).expect("invalid input");
        let res = get_biggest_rectangle(&grid);

        assert_eq!(res, 50);
    }

    #[rstest]
    #[test_log::test]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let grid = parse(&puzzle_input).expect("invalid input");
        let res = get_biggest_rectangle(&grid);

        assert_eq!(res, 4752484112);
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
