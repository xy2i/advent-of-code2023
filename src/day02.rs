use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum BallKind {
    Blue,
    Red,
    Green,
}

type Record = HashMap<BallKind, u8>;
type Game = Vec<Record>;

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| {
            let game_delim = l.find(":").unwrap();
            let l = &l[game_delim + 1..];
            let mut game = vec![];

            let records = l.split(";");
            for record in records {
                let mut h = HashMap::new();
                let record = record.split(",");
                for entry in record {
                    let entry = entry.trim();
                    let mut entry = entry.split(" ");
                    let number = entry.next().unwrap().parse::<u8>().unwrap();
                    let color = match entry.next().unwrap().chars().nth(0).unwrap() {
                        'r' => BallKind::Red,
                        'b' => BallKind::Blue,
                        'g' => BallKind::Green,
                        _ => unreachable!(),
                    };
                    h.insert(color, number);
                }
                game.push(h);
            }
            game
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Vec<Game>) -> u32 {
    const NB_RED: u8 = 12;
    const NB_GREEN: u8 = 13;
    const NB_BLUE: u8 = 14;
    input
        .iter()
        .enumerate()
        .map(|(i, game)| {
            if game.iter().any(|record| {
                record.get(&BallKind::Red).unwrap_or(&0) > &NB_RED
                    || record.get(&BallKind::Green).unwrap_or(&0) > &NB_GREEN
                    || record.get(&BallKind::Blue).unwrap_or(&0) > &NB_BLUE
            }) {
                0
            } else {
                (i + 1) as u32
            }
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &Vec<Game>) -> u32 {
    input
        .iter()
        .map(|game| {
            let mut h = HashMap::new();
            for record in game {
                for (k, &v) in record {
                    h.entry(k)
                        .and_modify(|old_v| {
                            if *old_v < v as u32 {
                                *old_v = v as u32
                            }
                        })
                        .or_insert(v as u32);
                }
            }

            h.values().product::<u32>()
        })
        .sum()
}

#[test]
fn day2_p1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(part1(&generator(input)), 8);
}

#[test]
fn day2_p2() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(part2(&generator(input)), 2286)
}
