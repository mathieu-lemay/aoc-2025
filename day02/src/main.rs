use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::{format_duration, get_input_as_string, tracing_init};
use itertools::Itertools;

fn main() {
    tracing_init();

    let input = get_input_as_string("day02.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_str());

    let t = start.elapsed().as_nanos();

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {}", format_duration(t));
}

struct Range {
    start: u64,
    end: u64,
}

#[tracing::instrument(skip_all)]
fn solve(input: &str) -> (impl Display, impl Display) {
    let ranges = parse_ranges(input);

    let p1 = get_sum_of_invalid_ids(&ranges);
    let p2 = get_sum_of_real_invalid_ids(&ranges);

    (p1, p2)
}

#[tracing::instrument(skip_all)]
fn parse_ranges(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|r| {
            let (s, e) = r
                .splitn(2, '-')
                .map(|i| i.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();

            Range { start: s, end: e }
        })
        .collect_vec()
}

#[tracing::instrument(skip_all)]
fn get_sum_of_invalid_ids(ranges: &[Range]) -> u64 {
    let mut sum = 0;

    for r in ranges {
        let mut v = r.start;
        while v <= r.end {
            let len = v.checked_ilog10().unwrap_or(0) + 1;

            if !len.is_multiple_of(2) {
                // Skip straight to the next even length value
                v = 10u64.pow(len);
                continue;
            }

            let div = 10u64.pow(len / 2);

            let hi = v / div;
            let lo = v % div;

            if hi == lo {
                sum += v;
            }

            v += 1;
        }
    }

    sum
}

fn get_divisors(cache: &mut HashMap<u32, Vec<u32>>, n: u32) -> &[u32] {
    cache.entry(n).or_insert_with(|| {
        let mut factors: Vec<u32> = Vec::new();

        for i in 1..=(n / 2) {
            if n.is_multiple_of(i) {
                factors.push(i);
            }
        }

        factors
    })
}

#[tracing::instrument(skip_all)]
fn get_sum_of_real_invalid_ids(ranges: &[Range]) -> u64 {
    let mut sum = 0;

    let mut div_cache = HashMap::new();

    for r in ranges {
        for v in r.start..=r.end {
            let len = v.checked_ilog10().unwrap_or(0) + 1;

            let divisors = get_divisors(&mut div_cache, len);

            if divisors.iter().any(|&d| is_repeating(v, len, d)) {
                sum += v;
            }
        }
    }

    sum
}

fn is_repeating(n: u64, l: u32, chunk_size: u32) -> bool {
    let mut div_size = 0;

    let div = 10u64.pow(chunk_size);
    let reference = n % div;

    div_size += chunk_size;
    while div_size < l {
        let new_div = 10u64.pow(div_size);

        let val = (n / new_div) % div;
        if val != reference {
            return false;
        }

        div_size += chunk_size;
    }

    true
}

#[cfg(test)]
mod tests {
    use aoc_common::parse_test_input_as_string;
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn test_input() -> String {
        parse_test_input_as_string(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        )
    }

    #[fixture]
    fn puzzle_input() -> String {
        get_input_as_string("day02.txt")
    }

    #[rstest]
    fn test_p1(test_input: String) {
        let ranges = parse_ranges(&test_input);
        let res = get_sum_of_invalid_ids(&ranges);

        assert_eq!(res, 1227775554);
    }

    #[rstest]
    fn test_p1_full_input(puzzle_input: String) {
        let ranges = parse_ranges(&puzzle_input);
        let res = get_sum_of_invalid_ids(&ranges);

        assert_eq!(res, 17077011375);
    }

    #[rstest]
    fn test_p2(test_input: String) {
        let ranges = parse_ranges(&test_input);
        let res = get_sum_of_real_invalid_ids(&ranges);

        assert_eq!(res, 4174379265);
    }

    #[rstest]
    fn test_p2_full_input(puzzle_input: String) {
        let ranges = parse_ranges(&puzzle_input);
        let res = get_sum_of_real_invalid_ids(&ranges);

        assert_eq!(res, 36037497037);
    }

    #[rstest]
    fn test_is_repeating() {
        assert!(is_repeating(22222, 5, 1));
        assert!(is_repeating(123123123, 9, 3));
        assert!(is_repeating(565656, 6, 2));

        assert!(!is_repeating(123123123, 9, 1));
        assert!(!is_repeating(565656, 6, 1));
        assert!(!is_repeating(565656, 6, 3));
    }

    #[rstest]
    fn test_get_divisors() {
        let mut cache = HashMap::new();

        assert_eq!(vec![1, 2], get_divisors(&mut cache, 4));
        assert_eq!(vec![1], get_divisors(&mut cache, 5));
        assert_eq!(vec![1, 2, 3], get_divisors(&mut cache, 6));
        assert_eq!(vec![1, 2, 3, 4, 6], get_divisors(&mut cache, 12));
    }
}
