use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input, tracing_init};

fn main() {
    tracing_init();

    let input = get_input("day03.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

#[tracing::instrument(skip_all)]
fn solve(input: &[String]) -> (impl Display, impl Display) {
    let banks = parse_banks(input);

    let p1 = get_total_joltage(&banks, 2);
    let p2 = get_total_joltage(&banks, 12);

    (p1, p2)
}

#[tracing::instrument(skip_all)]
fn parse_banks(input: &[String]) -> Vec<Vec<u8>> {
    input
        .iter()
        .map(|b| b.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[tracing::instrument(skip_all)]
fn get_total_joltage(input: &[Vec<u8>], n: u32) -> u64 {
    input.iter().map(|b| get_highest_joltage(b, n)).sum()
}

fn get_highest_joltage(bank: &[u8], n: u32) -> u64 {
    let bank_size = bank.len();
    let mut start = 0;
    let mut joltage: u64 = 0;

    for i in 0..n {
        let skip_end = n - i - 1;
        let mut m = 0;
        let mut idx = 0;

        for (i, &v) in bank
            .iter()
            .enumerate()
            .skip(start)
            .take(bank_size - start - skip_end as usize)
        {
            if v > m {
                m = v;
                idx = i;
            }
            if v == 9 {
                break;
            }
        }

        start = idx + 1;

        joltage = joltage * 10 + m as u64;
    }

    joltage
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
            987654321111111
            811111111111119
            234234234234278
            818181911112111
            ",
        )
    }

    #[fixture]
    fn puzzle_input() -> Vec<String> {
        get_input("day03.txt")
    }

    #[rstest]
    fn test_p1(test_input: Vec<String>) {
        let banks = parse_banks(&test_input);
        let res = get_total_joltage(&banks, 2);

        assert_eq!(res, 357);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: Vec<String>) {
        let banks = parse_banks(&puzzle_input);
        let res = get_total_joltage(&banks, 2);

        assert_eq!(res, 17766);
    }

    #[rstest]
    fn test_p2(test_input: Vec<String>) {
        let banks = parse_banks(&test_input);
        let res = get_total_joltage(&banks, 12);

        assert_eq!(res, 3121910778619);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: Vec<String>) {
        let banks = parse_banks(&puzzle_input);
        let res = get_total_joltage(&banks, 12);

        assert_eq!(res, 176582889354075);
    }

    #[rstest]
    #[case(vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1,], 2, 98)]
    #[case(vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9,], 2, 89)]
    #[case(vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8,], 2, 78)]
    #[case(vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1,], 2, 92)]
    #[case(vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1,], 12, 987654321111)]
    #[case(vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9,], 12, 811111111119)]
    #[case(vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8,], 12, 434234234278)]
    #[case(vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1,], 12, 888911112111)]
    fn test_get_highest_joltage_2(#[case] bank: Vec<u8>, #[case] n: u32, #[case] expected: u64) {
        assert_eq!(get_highest_joltage(&bank, n), expected);
    }
}
