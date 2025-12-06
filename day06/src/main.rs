use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};

fn main() {
    tracing_init();

    let input = get_input("day06.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let problems = parse_problems(input).expect("Failed to parse problems");
    let p1 = get_sum_of_results(&problems);
    let problems = parse_problems_like_a_dumbass(input).expect("Failed to parse problems");
    let p2 = get_sum_of_results(&problems);

    (p1, p2)
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Mul,
}

impl TryFrom<&str> for Op {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err(format!("Unknown operator: {}", value)),
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    values: Vec<u64>,
    op: Op,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.op {
            Op::Add => self.values.iter().copied().sum(),
            Op::Mul => self.values.iter().copied().product(),
        }
    }
}

#[tracing::instrument(skip_all)]
fn parse_problems(input: &[String]) -> Result<Vec<Problem>, String> {
    let len = input.len();
    let ops = input
        .iter()
        .nth_back(0)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let n = ops.len();
    let mut problems = ops
        .iter()
        .map(|&s| {
            let op = Op::try_from(s)?;
            Ok(Problem {
                values: Vec::with_capacity(len - 1),
                op,
            })
        })
        .collect::<Result<Vec<Problem>, String>>()?;

    for entry in input.iter().take(len - 1) {
        let values = entry
            .split(' ')
            .filter_map(|s| {
                if s.is_empty() {
                    return None;
                }

                s.parse::<u64>().ok()
            })
            .collect::<Vec<u64>>();

        if values.len() != n {
            return Err(format!("Invalid number of values: {}", values.len()));
        }

        for (i, &v) in values.iter().enumerate() {
            problems[i].values.push(v);
        }
    }

    Ok(problems)
}

#[tracing::instrument(skip_all)]
fn parse_problems_like_a_dumbass(input: &[String]) -> Result<Vec<Problem>, String> {
    let h = input.len();
    let w = input.iter().map(|r| r.len()).max().unwrap();

    let mut cols = (0..w)
        .map(|_| Vec::with_capacity(h))
        .collect::<Vec<Vec<char>>>();

    for r in input {
        r.chars()
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| {
                cols[i].push(c);
            })
    }

    let mut problems = Vec::with_capacity(w);
    let mut problem = Problem {
        values: Vec::with_capacity(h - 1),
        op: Op::Add,
    };

    for col in cols {
        if col.is_empty() {
            problems.push(problem.clone());
            problem = Problem {
                values: Vec::with_capacity(h - 1),
                op: Op::Add,
            };
            continue;
        }

        let mut val:u64 = 0;

        for c in &col {
            if c.is_ascii_digit() {
                val = val * 10 + c.to_digit(10).unwrap() as u64;
            }
        }
        problem.values.push(val);

        if *col.iter().last().unwrap() == '*' {
            problem.op = Op::Mul;
        }
    }

    problems.push(problem);

    Ok(problems)
}

#[tracing::instrument(skip_all)]
fn get_sum_of_results(problems: &[Problem]) -> u64 {
    problems.iter().map(Problem::solve).sum()
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
            123 328  51 64
             45 64  387 23
              6 98  215 314
            *   +   *   +
        ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day06.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let problems = parse_problems(&test_input).expect("Failed to parse test input");
        let res = get_sum_of_results(&problems);

        assert_eq!(res, 4277556);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let problems = parse_problems(&puzzle_input).expect("Failed to parse test input");
        let res = get_sum_of_results(&problems);

        assert_eq!(res, 5784380717354);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let problems =
            parse_problems_like_a_dumbass(&test_input).expect("Failed to parse test input");
        let res = get_sum_of_results(&problems);

        assert_eq!(res, 3263827);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let problems =
            parse_problems_like_a_dumbass(&puzzle_input).expect("Failed to parse test input");
        let res = get_sum_of_results(&problems);

        assert_eq!(res, 7996218225744);
    }
}
