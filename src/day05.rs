use std::{cmp, collections::BTreeMap, mem::swap, ops::Range};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::numbers_space;

#[derive(Debug)]
struct MapEntry {
    dst: u32,
    src: u32,
    len: u32,
}

impl MapEntry {
    fn parse(input: &str) -> Self {
        let mut iter = numbers_space(input);
        let dst = iter.next().unwrap();
        let src = iter.next().unwrap();
        let len = iter.next().unwrap();
        Self { dst, src, len }
    }
}

#[derive(Debug, Clone)]
struct Map {
    set: BTreeMap<u32, u32>,
}

impl Default for Map {
    fn default() -> Self {
        let mut set = BTreeMap::new();
        set.insert(0, 0);
        Self { set }
    }
}

impl Map {
    fn map(&self, x: u32) -> u32 {
        let (src, dst) = self.set.range(..=x).next_back().unwrap();
        let diff = x - src;
        dst + diff
    }

    fn map_ranges(&self, input: &mut Vec<Range<u32>>, output: &mut Vec<Range<u32>>) {
        for mut r in input.drain(..) {
            while !r.is_empty() {
                println!("{r:?}");
                let (&src1, &dst1) = self.set.range(..=r.start).next_back().unwrap();
                let output2 = self.set.range(r.start + 1..r.end).next();

                let diff = dst1.wrapping_sub(src1);
                let start = r.start.wrapping_add(diff);

                let end = match output2 {
                    None => r.end,
                    Some((&src2, _)) => src2,
                };

                r.start = end;
                let end = end.wrapping_add(diff);
                output.push(start..end);
            }
        }
    }

    fn push(&mut self, entry: MapEntry) {
        let end = entry.src.saturating_add(entry.len);
        self.set.entry(end).or_insert(end);
        self.set.insert(entry.src, entry.dst);
    }
}

pub struct Input {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Input {
    let input = &input[input.find(":").unwrap() + 1..];
    let mut input = input.lines();

    let seeds: Vec<u32> = crate::numbers_space(input.next().unwrap()).collect();

    let mut maps = vec![];
    for line in input {
        if line.trim().is_empty() {
            continue;
        }
        let first = line.chars().nth(0).unwrap();
        if first < '0' || first > '9' {
            maps.push(Map::default());
            continue;
        }

        let current_textmap = maps.iter_mut().last().unwrap();
        current_textmap.push(MapEntry::parse(line));
    }

    Input { seeds, maps }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> u32 {
    input
        .seeds
        .iter()
        .map(|&seed| input.maps.iter().fold(seed, |acc, map| map.map(acc)))
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> u32 {
    let mut ranges1 = input
        .seeds
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect::<Vec<_>>();
    let mut ranges2 = Vec::with_capacity(ranges1.len());

    let map = &input.maps;
    map[0].map_ranges(&mut ranges1, &mut ranges2);
    map[1].map_ranges(&mut ranges2, &mut ranges1);
    map[2].map_ranges(&mut ranges1, &mut ranges2);
    map[3].map_ranges(&mut ranges2, &mut ranges1);
    map[4].map_ranges(&mut ranges1, &mut ranges2);
    map[5].map_ranges(&mut ranges2, &mut ranges1);
    map[6].map_ranges(&mut ranges1, &mut ranges2);

    ranges2.into_iter().map(|r| r.start).min().unwrap()
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
