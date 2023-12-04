use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<(HashSet<u16>, Vec<u16>)> {
    input
        .lines()
        .map(|l| {
            let mut l = l[l.find(":").unwrap() + 1..].split("|");
            let winning = l.next().unwrap();
            let ours = l.next().unwrap();

            let mut set = HashSet::new();
            for number in crate::numbers_space(winning) {
                set.insert(number);
            }
            let mut v = vec![];
            for number in crate::numbers_space(ours) {
                v.push(number);
            }
            (set, v)
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &Vec<(HashSet<u16>, Vec<u16>)>) -> u16 {
    input
        .iter()
        .map(|(winning, ours)| {
            ours.iter().fold(0u16, |sum, number| {
                if winning.contains(number) {
                    if sum == 0 {
                        1
                    } else {
                        sum * 2
                    }
                } else {
                    sum
                }
            })
        })
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &Vec<(HashSet<u16>, Vec<u16>)>) -> usize {
    let mut dp = vec![1; input.len()];

    for (i, (winning, ours)) in input.iter().enumerate() {
        let matching: usize = ours.iter().filter(|&x| winning.contains(x)).count();

        for j in i + 1..i + 1 + matching {
            dp[j] += dp[i]
        }
    }

    dp.iter().sum()
}

#[test]
fn day4() {
    let i = &parse(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    );
    assert_eq!(part1(i), 13);
    assert_eq!(part2(i), 30);
}
