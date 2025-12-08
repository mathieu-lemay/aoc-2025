use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::grid::Grid;
use aoc_common::{Point, format_duration, get_input, tracing_init};
use tracing::debug;

fn main() {
    tracing_init();

    let input = get_input("day07.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let grid: Grid<Element> = Grid::from_input(input).expect("unable to parse input");

    let p1 = get_number_of_splits(&mut grid.clone());
    let p2 = get_timelines(&grid);

    (p1, p2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    Start,
    Splitter,
    Beam,
    Empty,
}

impl TryFrom<char> for Element {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Element::Start),
            '^' => Ok(Element::Splitter),
            '|' => Ok(Element::Beam),
            '.' => Ok(Element::Empty),
            e => Err(format!("invalid element {:?}", e)),
        }
    }
}

impl From<Element> for char {
    fn from(value: Element) -> Self {
        match value {
            Element::Start => 'S',
            Element::Splitter => '^',
            Element::Beam => '|',
            Element::Empty => '.',
        }
    }
}

#[tracing::instrument(skip_all)]
fn get_number_of_splits(grid: &mut Grid<Element>) -> usize {
    let mut splits = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let pos = Point::new(x, y);
            let cur = grid.get(pos);

            debug!(
                "pos={:?}, cur={:?}, above={:?}",
                pos,
                cur,
                grid.get_above(pos)
            );

            match cur {
                Element::Start => {
                    let p = Point::new(x, y + 1);
                    grid.set(p, Element::Beam);
                }
                Element::Empty => {
                    if let Some(Element::Beam) = grid.get_above(pos) {
                        debug!("above is beam, setting {:?} to beam", pos);
                        grid.set(pos, Element::Beam);
                    }
                }
                Element::Splitter => {
                    if let Some(Element::Beam) = grid.get_above(pos) {
                        splits += 1;
                        if pos.x > 0 {
                            let p = Point::new(x - 1, y);
                            grid.set(p, Element::Beam);
                        }
                        if pos.x < grid.width - 1 {
                            let p = Point::new(x + 1, y);
                            grid.set(p, Element::Beam);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    splits
}

#[tracing::instrument(skip_all)]
fn get_timelines(grid: &Grid<Element>) -> usize {
    let x = (0..grid.width)
        .find(|&x| grid.get(Point::new(x, 0)) == Element::Start)
        .unwrap();

    let mut cache: HashMap<Point<usize>, usize> = HashMap::new();

    get_children(grid, &mut cache, x, 0)
}

fn get_children(
    grid: &Grid<Element>,
    cache: &mut HashMap<Point<usize>, usize>,
    x: usize,
    y: usize,
) -> usize {
    let pos = Point::new(x, y);
    if let Some(&n) = cache.get(&pos) {
        return n;
    }

    if y >= grid.height {
        return 1;
    }

    let cur = grid.get(pos);
    let timelines = match cur {
        Element::Start | Element::Empty => get_children(grid, cache, x, y + 1),
        Element::Splitter => {
            let mut t = 0;
            if x > 0 {
                t += get_children(grid, cache, x - 1, y + 1);
            }
            if x < grid.width - 1 {
                t += get_children(grid, cache, x + 1, y + 1);
            }
            t
        }
        _ => 0,
    };

    cache.insert(pos, timelines);

    timelines
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
            .......S.......
            ...............
            .......^.......
            ...............
            ......^.^......
            ...............
            .....^.^.^.....
            ...............
            ....^.^...^....
            ...............
            ...^.^...^.^...
            ...............
            ..^...^.....^..
            ...............
            .^.^.^.^.^...^.
            ...............
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day07.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let mut grid: Grid<Element> = Grid::from_input(&test_input).expect("unable to parse input");
        let res = get_number_of_splits(&mut grid);

        assert_eq!(res, 21);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let mut grid: Grid<Element> =
            Grid::from_input(&puzzle_input).expect("unable to parse input");
        let res = get_number_of_splits(&mut grid);

        assert_eq!(res, 1585);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let grid: Grid<Element> = Grid::from_input(&test_input).expect("unable to parse input");
        let res = get_timelines(&grid);

        assert_eq!(res, 40);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let grid: Grid<Element> = Grid::from_input(&puzzle_input).expect("unable to parse input");
        let res = get_timelines(&grid);

        assert_eq!(res, 16716444407407);
    }
}
