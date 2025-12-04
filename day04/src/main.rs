use std::fmt::Display;
use std::time::Instant;

use aoc_common::{Point, format_duration, get_input, tracing_init};
use grid::Grid;
use tracing::debug;

mod grid;

fn main() {
    tracing_init();

    let input = get_input("day04.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Element {
    Empty,
    RollOfPaper,
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            '.' => Element::Empty,
            '@' => Element::RollOfPaper,
            _ => panic!("Invalid value: {}", value),
        }
    }
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let mut grid: Grid<Element> = Grid::from_input(input).expect("invalid grid");
    let p1 = get_accessible_rolls(&grid);
    let p2 = get_accessible_rolls_recursive(&mut grid);

    (p1, p2)
}

#[tracing::instrument(skip_all)]
fn get_accessible_rolls(grid: &Grid<Element>) -> i32 {
    let mut count = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point::new(x, y);
            let elem = grid.get(p);
            if elem != Element::RollOfPaper {
                debug!("{:?}: Not a roll of paper", p);
                continue;
            }

            let adjacent_cells = grid.get_neighbors(p);

            let n_adjacent_rolls = adjacent_cells
                .iter()
                .filter(|c| c.value == Element::RollOfPaper)
                .count();
            debug!(
                "{:?}: Has {} adjacent cells, of which {} are rolls",
                p,
                adjacent_cells.len(),
                n_adjacent_rolls
            );

            if n_adjacent_rolls < 4 {
                count += 1;
            }
        }
    }

    count
}

#[tracing::instrument(skip_all)]
fn get_accessible_rolls_recursive(grid: &mut Grid<Element>) -> i32 {
    let mut count = 0;

    loop {
        let mut pass_count = 0;
        for y in 0..grid.height {
            for x in 0..grid.width {
                let p = Point::new(x, y);
                let elem = grid.get(p);
                if elem != Element::RollOfPaper {
                    debug!("{:?}: Not a roll of paper", p);
                    continue;
                }

                let adjacent_cells = grid.get_neighbors(p);

                let n_adjacent_rolls = adjacent_cells
                    .iter()
                    .filter(|c| c.value == Element::RollOfPaper)
                    .count();

                debug!(
                    "{:?}: Has {} adjacent cells, of which {} are rolls",
                    p,
                    adjacent_cells.len(),
                    n_adjacent_rolls
                );

                if n_adjacent_rolls < 4 {
                    pass_count += 1;
                    grid.set(p, Element::Empty);
                }
            }
        }

        if pass_count == 0 {
            break;
        }

        count += pass_count;
    }

    count
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
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day04.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let grid: Grid<Element> = Grid::from_input(&test_input).unwrap();
        let res = get_accessible_rolls(&grid);

        assert_eq!(res, 13);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let grid: Grid<Element> = Grid::from_input(&puzzle_input).unwrap();
        let res = get_accessible_rolls(&grid);

        assert_eq!(res, 1480);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let mut grid: Grid<Element> = Grid::from_input(&test_input).unwrap();
        let res = get_accessible_rolls_recursive(&mut grid);

        assert_eq!(res, 43);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let mut grid: Grid<Element> = Grid::from_input(&puzzle_input).unwrap();
        let res = get_accessible_rolls_recursive(&mut grid);

        assert_eq!(res, 8899);
    }
}
