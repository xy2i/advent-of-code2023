use crate::*;

#[aoc(day1, part1, Chars)]
fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap());
            let second = line
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap());

            match (first, second) {
                (Some(first), Some(second)) => first * 10 + second,
                _ => 0,
            }
        })
        .sum()
}

#[aoc(day1, part2, Chars)]
fn solve2(input: &str) -> u32 {
    let scan = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let vals = [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    input
        .lines()
        .map(|line| {
            let mut best_first = (usize::MAX, 0);
            for (pos, pat) in scan.iter().enumerate() {
                let Some(i) = line.find(pat) else { continue };
                if best_first.0 > i {
                    best_first = (i, vals[pos]);
                }
            }

            let mut best_last = (0, 0);
            for (pos, pat) in scan.iter().enumerate() {
                let Some(i) = line.rfind(pat) else { continue };
                if best_last.0 <= i {
                    best_last = (i, vals[pos]);
                }
            }

            best_first.1 * 10 + best_last.1
        })
        .sum()
}
