use std::iter;

use crate::get_ints;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Race {
    t: u64,
    d: u64,
}

impl Race {
    fn solve(&self) -> u64 {
        let &Self { t, d } = self;
        /* (t-i)*i > d
        ti - i^2 > d
        i^2 - ti < d
        i^2 - ti - d < 0

        i^2 - it - d < 0
        a=1, b=-t, c=-d
        */

        let di = t * t - 4 * d;
        let di = (di as f64).sqrt() - 2.;

        let r1 = ((t as f64 + di) / 2.).ceil();
        let r2 = ((t as f64 - di) / 2.).floor();

        (r1 - r2) as u64 + 1
    }
}

#[aoc_generator(day6, part1)]
pub fn parse(input: &[u8]) -> Vec<Race> {
    let mut lines = input.split(|&b| b == b'\n');

    let times = get_ints(lines.next().unwrap());
    let distances = get_ints(lines.next().unwrap());

    iter::zip(times, distances)
        .map(|(t, d)| Race {
            t: t.try_into().unwrap(),
            d: d.try_into().unwrap(),
        })
        .collect()
}
fn parse_num(v: &[u8]) -> u64 {
    let mut n = 0;
    for &b in v {
        if b.is_ascii_digit() {
            n = n * 10 + u64::from(b & 0xf);
        }
    }

    n
}

#[aoc_generator(day6, part2)]
pub fn parse2(input: &[u8]) -> Race {
    let mut lines = input.split(|&b| b == b'\n');

    let t = parse_num(lines.next().unwrap());
    let d = parse_num(lines.next().unwrap());

    Race { t, d }
}

#[aoc(day6, part1)]
pub fn part1(input: &Vec<Race>) -> u64 {
    input.iter().map(Race::solve).product()
}

#[aoc(day6, part2)]
pub fn part2(input: &Race) -> u64 {
    input.solve()
}

#[test]
pub fn day6() {
    let i = b"Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(part1(&parse(i)), 288);
    assert_eq!(part2(&parse2(i)), 71503);
}
