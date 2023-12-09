use aoc_runner_derive::aoc;

use crate::numbers_space;

#[aoc(day9, part1)]
pub fn run(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            fn rec(slice: &[i64]) -> i64 {
                let iter = slice.windows(2).map(|xs| xs[1] - xs[0]);

                let found = iter.clone().all(|x| x == 0);
                if found {
                    return *slice.last().unwrap();
                } else {
                    let new = iter.collect::<Vec<_>>();
                    return slice.last().unwrap() + rec(&new);
                }
            }

            let slice: Vec<i64> = numbers_space(line).collect();
            let a = rec(&slice);
            a
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn run2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            fn rec(slice: &[i64]) -> i64 {
                let iter = slice.windows(2).map(|xs| xs[1] - xs[0]);

                let found = iter.clone().all(|x| x == 0);
                if found {
                    return *slice.last().unwrap();
                } else {
                    let new = iter.collect::<Vec<_>>();
                    return slice.last().unwrap() + rec(&new);
                }
            }

            let mut slice: Vec<i64> = numbers_space(line).collect();
            slice.reverse();
            let a = rec(&slice);
            a
        })
        .sum()
}

#[test]
pub fn test() {
    assert_eq!(
        run("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"),
        114
    );
    assert_eq!(
        run2(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
        ),
        2
    );
}
