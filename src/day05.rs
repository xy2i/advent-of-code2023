use std::cmp;

use aoc_runner_derive::{aoc, aoc_generator};
use rangemap::RangeMap;

#[derive(Debug)]
struct TextMapEntry {
    dest_range_start: u32,
    source_range_start: u32,
    range_len: u32,
}

type TextMap = Vec<TextMapEntry>;

struct Input {
    seeds: Vec<u32>,
    textmaps: Vec<TextMap>,
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Input {
    let input = &input[input.find(":").unwrap() + 1..];
    let mut input = input.lines();

    let seeds: Vec<u32> = crate::numbers_space(input.next().unwrap()).collect();

    let mut textmaps = vec![];
    for line in input {
        if line.trim().is_empty() {
            continue;
        }
        let first = line.chars().nth(0).unwrap();
        if first < '0' || first > '9' {
            textmaps.push(vec![]);
            continue;
        }

        let current_textmap = textmaps.iter_mut().last().unwrap();
        let mut iter = crate::numbers_space(line);
        current_textmap.push(TextMapEntry {
            dest_range_start: iter.next().unwrap(),
            source_range_start: iter.next().unwrap(),
            range_len: iter.next().unwrap(),
        })
    }

    Input { seeds, textmaps }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> i64 {
    let rangemaps = input
        .textmaps
        .iter()
        .map(|textmap| {
            let mut map = RangeMap::new();
            for entry in textmap {
                map.insert(
                    entry.source_range_start as i64
                        ..entry.source_range_start as i64 + entry.range_len as i64,
                    entry.dest_range_start as i64 - entry.source_range_start as i64,
                )
            }
            map
        })
        .collect::<Vec<_>>();

    let mut low = i64::MAX;
    for &number in &input.seeds {
        let mut number = number as i64;
        for range_map in &rangemaps {
            let &offset = range_map.get(&number).unwrap_or(&0);
            number += offset;
        }
        low = cmp::min(low, number);
    }
    low
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> i64 {
    let rangemaps = input
        .textmaps
        .iter()
        .map(|textmap| {
            let mut map = RangeMap::new();
            for entry in textmap {
                map.insert(
                    entry.source_range_start as i64
                        ..entry.source_range_start as i64 + entry.range_len as i64,
                    entry.dest_range_start as i64 - entry.source_range_start as i64,
                )
            }
            map
        })
        .collect::<Vec<_>>();

    let mut low = i64::MAX;
    for pair in input.seeds.chunks(2) {
        let number = pair[0] as i64;
        let len = pair[1] as i64;

        for mut number in number..number + len {
            for range_map in &rangemaps {
                let &offset = range_map.get(&number).unwrap_or(&0);
                number += offset;
            }
            low = cmp::min(low, number);
        }
    }
    low
}

#[test]
pub fn day5() {
    let input = &parse(
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
    );
    assert_eq!(part1(input), 35);
    assert_eq!(part2(input), 46);
}
