use std::fmt::Display;
use std::time::Instant;

use aoc_common::{Point, format_duration, get_input, tracing_init};
use geo::algorithm::contains::Contains;
use geo::{Coord, LineString, Polygon, Rect, coord};
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
    let p2 = get_biggest_rectangle_in_grid(&points);

    (p1, p2)
}

fn get_rectangle_size(p1: &Point<usize>, p2: &Point<usize>) -> usize {
    ((p1.x as i64 - p2.x as i64).abs() + 1) as usize
        * ((p1.y as i64 - p2.y as i64).abs() + 1) as usize
}

#[tracing::instrument(skip_all)]
fn parse(input: &[String]) -> Result<Vec<Point<usize>>, String> {
    let mut points = Vec::new();
    for l in input {
        if let Some((x, y)) = l.split(',').collect_tuple() {
            let x = x
                .parse::<usize>()
                .map_err(|e| format!("invalid entry: {}: {}", l, e))?;
            let y = y
                .parse::<usize>()
                .map_err(|e| format!("invalid entry: {}: {}", l, e))?;
            points.push(Point::new(x, y));
        } else {
            return Err(format!("invalid entry: {}", l));
        }
    }

    Ok(points)
}

#[tracing::instrument(skip_all)]
fn get_biggest_rectangle(points: &[Point<usize>]) -> usize {
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

#[tracing::instrument(skip_all)]
fn get_biggest_rectangle_in_grid(points: &[Point<usize>]) -> usize {
    let polygon = get_polygon(points);

    let n = points.len() * points.len() / 2;
    let mut c = 0;
    let mut pct = 0;

    let mut max = 0;

    for (i, a) in points.iter().enumerate() {
        for b in points.iter().skip(i + 1) {
            c += 1;
            let p = c * 100 / n;
            if p != pct {
                debug!("{} / {} ({}%)", c, n, p);
                pct = p;
            }

            if is_in_polygon(a, b, &polygon) {
                let size = get_rectangle_size(a, b);
                if size > max {
                    debug!("Found bigger rectangle with {} and {}: {}", a, b, size);
                    max = size;
                }
            }
        }
    }

    max
}

fn is_in_polygon(a: &Point<usize>, b: &Point<usize>, p: &Polygon) -> bool {
    if a.x == b.x || a.y == b.y {
        return false;
    }
    let c1 = coord! {x: a.x as f64, y: a.y as f64};
    let c2 = coord! {x: b.x as f64, y: b.y as f64};
    let r = Rect::new(c1, c2);

    p.contains(&r)
}

#[tracing::instrument(skip_all)]
fn get_polygon(points: &[Point<usize>]) -> Polygon {
    let ls = LineString::from(
        points
            .iter()
            .map(|p| coord! {x: p.x as f64, y: p.y as f64})
            .collect::<Vec<Coord<f64>>>(),
    );

    Polygon::new(ls, vec![])
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
        let points = parse(&test_input).expect("invalid input");
        let res = get_biggest_rectangle(&points);

        assert_eq!(res, 50);
    }

    #[rstest]
    #[test_log::test]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let points = parse(&puzzle_input).expect("invalid input");
        let res = get_biggest_rectangle(&points);

        assert_eq!(res, 4752484112);
    }

    #[rstest]
    #[test_log::test]
    fn test_p2(test_input: Vec<String>) {
        let points = parse(&test_input).expect("invalid input");
        let res = get_biggest_rectangle_in_grid(&points);

        assert_eq!(res, 24);
    }

    #[rstest]
    #[test_log::test]
    #[ignore] // Quite slow
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let points = parse(&puzzle_input).expect("invalid input");
        let res = get_biggest_rectangle_in_grid(&points);

        assert_eq!(res, 1465767840);
    }
}
