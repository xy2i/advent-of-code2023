use aoc_runner_derive::{aoc, aoc_generator};

use crate::parse_num;

#[derive(Debug, PartialEq, Eq)]
pub struct Card(u8);

impl From<&u8> for Card {
    fn from(value: &u8) -> Self {
        Card(match value {
            b'A' => 12,
            b'K' => 11,
            b'Q' => 10,
            b'J' => 9,
            b'T' => 8,
            value if value.is_ascii_digit() => (value & 0xf) - 2,
            _ => panic!(),
        })
    }
}

impl Card {
    fn from_part2(value: &u8) -> Self {
        Card(match value {
            b'A' => 12,
            b'K' => 11,
            b'Q' => 10,
            b'T' => 9,
            value if value.is_ascii_digit() => (value & 0xf) - 1,
            b'J' => 0,
            _ => panic!(),
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Kind {
    High = 0,
    OnePair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

impl From<&Hand> for Kind {
    fn from(value: &Hand) -> Self {
        use Kind::*;
        match nb_same(value) {
            (5, _) => Five,
            (4, _) => Four,
            (3, 2) => Full,
            (3, _) => Three,
            (2, 2) => TwoPair,
            (2, _) => OnePair,
            _ => High,
        }
    }
}

impl Kind {
    fn from_part2(value: &Hand) -> Self {
        let kind = Kind::from(value);
        let Hand(a, b, c, d, e) = value;
        let joker_count = [a, b, c, d, e].iter().filter(|&x| x.0 == 0).count();

        if joker_count == 0 {
            return kind;
        }

        use Kind::*;
        match (kind, joker_count) {
            (Five, 5) => Five,
            (Four, 4) => Five,
            (Four, 1) => Five,
            // if full, only case with jokers can be jjjxx or jjxxx
            (Full, 3) => Five,
            (Full, 2) => Five,
            (Three, 3) => Four,
            (Three, 2) => Five,
            (Three, 1) => Four,
            (TwoPair, 2) => Four,
            (TwoPair, 1) => Full,
            // abcjj
            (OnePair, 2) => Three,
            (OnePair, 1) => Three,
            (High, 4) => Five,
            (High, 3) => Four,
            (High, 2) => Three,
            (High, 1) => OnePair,
            _ => panic!("{kind:?}, {joker_count}, {value:?}"),
        }
    }
}

#[derive(Debug)]
pub struct Hand(Card, Card, Card, Card, Card);

impl Hand {
    pub fn value(&self) -> u32 {
        let Hand(a, b, c, d, e) = self;
        ((Kind::from(self) as u32) << 20)
            + ((a.0 as u32) << 16)
            + ((b.0 as u32) << 12)
            + ((c.0 as u32) << 8)
            + ((d.0 as u32) << 4)
            + (e.0 as u32)
    }

    pub fn value_part2(&self) -> u32 {
        let Hand(a, b, c, d, e) = self;
        ((Kind::from_part2(self) as u32) << 20)
            + ((a.0 as u32) << 16)
            + ((b.0 as u32) << 12)
            + ((c.0 as u32) << 8)
            + ((d.0 as u32) << 4)
            + (e.0 as u32)
    }
}

#[aoc_generator(day7, part1)]
pub fn parse(input: &str) -> Vec<(Hand, u64)> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.as_bytes().iter();
            (
                Hand(
                    Card::from(chars.next().unwrap()),
                    Card::from(chars.next().unwrap()),
                    Card::from(chars.next().unwrap()),
                    Card::from(chars.next().unwrap()),
                    Card::from(chars.next().unwrap()),
                ),
                parse_num(&line.as_bytes()[5..]),
            )
        })
        .collect()
}

#[aoc_generator(day7, part2)]
pub fn parse2(input: &str) -> Vec<(Hand, u64)> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.as_bytes().iter();
            (
                Hand(
                    Card::from_part2(chars.next().unwrap()),
                    Card::from_part2(chars.next().unwrap()),
                    Card::from_part2(chars.next().unwrap()),
                    Card::from_part2(chars.next().unwrap()),
                    Card::from_part2(chars.next().unwrap()),
                ),
                parse_num(&line.as_bytes()[5..]),
            )
        })
        .collect()
}

// (most nb same, 2nd most nb same)
fn nb_same(hand: &Hand) -> (u8, u8) {
    let mut lut = [0u8; 13];
    lut[hand.0 .0 as usize] += 1;
    lut[hand.1 .0 as usize] += 1;
    lut[hand.2 .0 as usize] += 1;
    lut[hand.3 .0 as usize] += 1;
    lut[hand.4 .0 as usize] += 1;
    lut.sort_by(|a, b| b.cmp(a));
    (lut[0], lut[1])
}

#[aoc(day7, part1)]
pub fn part1(input: &[(Hand, u64)]) -> u64 {
    let mut hand_values = input
        .into_iter()
        .map(|(hand, bid)| (hand.value(), bid))
        .collect::<Vec<_>>();
    hand_values.sort_unstable();
    hand_values
        .iter()
        .enumerate()
        .map(|(i, &(_, bid))| bid * ((i as u64) + 1))
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &[(Hand, u64)]) -> u64 {
    let mut hand_values = input
        .into_iter()
        .map(|(hand, bid)| (hand.value_part2(), bid))
        .collect::<Vec<_>>();
    hand_values.sort_unstable();
    hand_values
        .iter()
        .enumerate()
        .map(|(i, &(_, bid))| bid * ((i as u64) + 1))
        .sum()
}

#[test]
fn day7() {
    let i = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(part1(&parse(i)), 6440);
    assert_eq!(part2(&parse2(i)), 5905);
}
