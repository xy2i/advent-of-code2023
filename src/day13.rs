use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Pattern {
    lines: Vec<Vec<char>>,
    cols: Vec<Vec<char>>,
}

pub fn mk_pattern(v: Vec<Vec<char>>) -> Pattern {
    let lines: Vec<Vec<char>> = v.clone();
    let mut cols = vec![];
    for j in 0..v[0].len() {
        let mut col = vec![];
        for i in 0..v.len() {
            col.push(v[i][j]);
        }
        cols.push(col);
    }

    Pattern { lines, cols }
}

pub fn mirror(entries: &Vec<Vec<char>>) -> u64 {
    let mut res = 0;
    for i in 1i32..=(entries.len() - 1) as i32 {
        let (mut left, mut right) = (i - 1, i);
        let mut mirrored = true;
        while left >= 0 && right <= (entries.len() - 1) as i32 {
            if entries[left as usize] != entries[right as usize] {
                mirrored = false;
                break;
            };
            left -= 1;
            right += 1;
        }
        if mirrored {
            res += i; // +1 because 1-indexed
            break;
        }
    }
    res as u64
}

pub fn mirror_smudge(entries: &Vec<Vec<char>>) -> u64 {
    let mut res = 0;
    for i in 1i32..=(entries.len() - 1) as i32 {
        let (mut left, mut right) = (i - 1, i);
        let mut mirrored = true;
        let mut smudge = false;
        while left >= 0 && right <= (entries.len() - 1) as i32 {
            for k in 0..entries[0].len() {
                if entries[left as usize][k] != entries[right as usize][k] {
                    if smudge {
                        mirrored = false;
                        break;
                    }
                    smudge = true;
                }
            }
            left -= 1;
            right += 1;
        }
        if mirrored && smudge {
            res += i; // +1 because 1-indexed
            break;
        }
    }
    res as u64
}

#[aoc(day13, part1)]
pub fn run(input: &str) -> u64 {
    let mut patterns = vec![];
    let mut tmp = vec![];
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(mk_pattern(tmp));
            tmp = vec![];
        } else {
            tmp.push(line.chars().collect::<Vec<_>>());
        }
    }
    patterns.push(mk_pattern(tmp));

    patterns
        .iter()
        .map(|pattern| {
            let mut res = 0;
            res += mirror(&pattern.lines) * 100;
            res += mirror(&pattern.cols);
            res
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn run2(input: &str) -> u64 {
    let mut patterns = vec![];
    let mut tmp = vec![];
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(mk_pattern(tmp));
            tmp = vec![];
        } else {
            tmp.push(line.chars().collect::<Vec<_>>());
        }
    }
    patterns.push(mk_pattern(tmp));

    patterns
        .iter()
        .map(|pattern| {
            let mut res = 0;
            res += mirror_smudge(&pattern.lines) * 100;
            res += mirror_smudge(&pattern.cols);
            res
        })
        .sum()
}

#[test]
pub fn day13() {
    assert_eq!(
        run("#..##..###..#
..#..#....#.#
#.####.##.#..
#.#..#.####.#
...##.....#.#
###..###.....
##.##.######.
..####.....##
##.##.#######
.##..##.##..#
.######......
.######......
.##..##.##..."),
        4
    );
    assert_eq!(
        run(".##
###"),
        2
    );
    assert_eq!(
        run("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"),
        405
    );
}
