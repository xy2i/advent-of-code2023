use std::iter;

use crate::get_ints;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Race {
    t: u32,
    d: u32,
}

impl Race {
    fn solve(Self { t, d }: &Self) -> u64 {
        /* (t-i)*i > d
        ti - i^2 > d
        i^2 - ti < d
        i^2 - ti - d < 0

        i^2 - it - d < 0
        a=1, b=-t, c=-d
        */
        let a = 1.;
        let b = -(f64::from(*t));
        let c = -(f64::from(*d));

        let di = b * b - 4. * a * c;

        let r1 = (-b + di.sqrt()) / (2. * a);
        let r2 = (-b - di.sqrt()) / (2. * a);

        dbg!(r1, r2);
        return r1 as u64;
    }
}

#[aoc_generator(day6, Bytes)]
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

#[aoc(day6, part1)]
pub fn part1(input: &Vec<Race>) -> u64 {
    input.iter().map(Race::solve).product()
}

#[test]
pub fn day6() {
    let i = &parse(
        b"Time:      7  15   30
Distance:  9  40  200",
    );
    assert_eq!(part1(i), 288);
}
