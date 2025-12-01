use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};
use tracing::debug;

fn main() {
    tracing_init();

    let input = get_input("day01.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let (p1, p2) = get_passwords(input);

    (p1, p2)
}

#[tracing::instrument(skip_all)]
fn get_passwords(input: &[String]) -> (i16, i16) {
    let mut count_exact = 0;
    let mut count_pass = 0;
    let mut pos: i16 = 50;

    for entry in input {
        let dir = entry.chars().next().unwrap();
        let amount: i16 = entry[1..].parse().unwrap();

        let prev = pos;

        if amount >= 100 {
            count_pass += amount / 100;
        }

        let rot = amount % 100;

        match dir {
            'L' => pos -= rot,
            'R' => pos += rot,
            _ => panic!("Invalid direction: {}", dir),
        }

        if (prev != 0 && pos < 0) || pos > 100 {
            count_pass += 1;
        }

        pos = (pos + 100) % 100;
        if pos == 0 {
            count_exact += 1;
        }

        debug!("The dial is rotated {}{} to point at {}.", dir, amount, pos);
    }

    (count_exact, count_pass + count_exact)
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
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day01.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let (res, _) = get_passwords(&test_input);

        assert_eq!(res, 3);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let (res, _) = get_passwords(&puzzle_input);

        assert_eq!(res, 1141);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let (_, res) = get_passwords(&test_input);

        assert_eq!(res, 6);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let (_, res) = get_passwords(&puzzle_input);

        assert_eq!(res, 6634);
    }
}
