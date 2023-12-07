use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

const SYMBOL: u8 = 10;
const DOT: u8 = 11;
const STAR: u8 = 12;

type Elem = u8;
type Mat = Vec<Vec<Elem>>;

trait ElemExt {
    fn is_symbol(self) -> bool;
    fn is_part_num(self) -> bool;
    fn is_star(self) -> bool;
}

impl ElemExt for Elem {
    fn is_symbol(self) -> bool {
        self == SYMBOL || self == STAR
    }
    fn is_part_num(self) -> bool {
        self < 10
    }
    fn is_star(self) -> bool {
        self == STAR
    }
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Mat {
    let mut matrix = vec![];
    for line in input.lines() {
        let mut line_vec = vec![];
        for s in line.chars() {
            line_vec.push(match s {
                s if s.is_ascii_digit() => s as u8 - '0' as u8,
                '.' => DOT,
                '*' => STAR,
                _ => SYMBOL,
            });
        }
        matrix.push(line_vec);
    }
    matrix
}

fn has_adjacent_symbol(matrix: &Mat, line: usize, start: usize, end: usize) -> bool {
    let nb_rows = matrix.len();
    let nb_cols = matrix[0].len();

    let line_start = if line == 0 { 0 } else { line - 1 };
    let col_start = if start == 0 { 0 } else { start - 1 };
    for i in line_start..=line + 1 {
        for j in col_start..=end + 1 {
            if i >= nb_rows || j >= nb_cols {
                continue;
            }
            if matrix[i][j].is_symbol() {
                return true;
            }
        }
    }
    false
}

#[aoc(day3, part2)]
fn solve2(matrix: &Mat) -> u32 {
    let mut stars = HashSet::new();
    let mut num_map: HashMap<usize, Vec<(u32, usize, usize)>> = HashMap::new();

    let mut sum = 0;
    for (i, line) in matrix.iter().enumerate() {
        let mut number: u32 = 0;
        let mut start: usize = 0;
        let mut found_number = false;
        for (j, &elem) in line.iter().enumerate() {
            if elem.is_star() {
                stars.insert((i, j));
            }

            if elem.is_part_num() {
                if !found_number {
                    found_number = true;
                    start = j
                }
                number = number * 10 + elem as u32
            }

            if !elem.is_part_num() && found_number {
                let end = j - 1;
                num_map
                    .entry(i)
                    .and_modify(|v| v.push((number, start, end)))
                    .or_insert(vec![(number, start, end)]);
                found_number = false;
                number = 0;
            }
        }
        if found_number {
            let end = line.len() - 1;
            num_map
                .entry(i)
                .and_modify(|v| v.push((number, start, end)))
                .or_insert(vec![(number, start, end)]);
        }
    }

    for (i, j) in stars {
        let (i, j) = (i as isize, j as isize);
        let mut close_nums = vec![];
        for scan_i in i - 1..=i + 1 {
            for _ in j - 1..=j + 1 {
                if scan_i < 0 {
                    continue;
                }
                let Some(nums_in_line) = num_map.get(&usize::try_from(scan_i).unwrap()) else {
                    continue;
                };
                for &(num, start, end) in nums_in_line {
                    let start = start as isize;
                    let end = end as isize;
                    if start < j - 1 && j + 1 <= end
                        || j - 1 <= end && end <= j + 1
                        || j - 1 <= start && start <= j + 1
                    {
                        if !close_nums.contains(&num) {
                            close_nums.push(num);
                        }
                    }
                }
            }
        }
        if close_nums.len() == 2 {
            sum += close_nums[0] * close_nums[1];
        }
    }
    sum
}

#[aoc(day3, part1)]
fn solve1(matrix: &Mat) -> u32 {
    let mut sum = 0;
    for (i, line) in matrix.iter().enumerate() {
        let mut number: u32 = 0;
        let mut start: usize = 0;
        let mut found_number = false;
        for (j, &elem) in line.iter().enumerate() {
            if elem.is_part_num() {
                if !found_number {
                    found_number = true;
                    start = j
                }
                number = number * 10 + elem as u32
            }

            if !elem.is_part_num() && found_number {
                let end = j - 1;
                if has_adjacent_symbol(&matrix, i, start, end) {
                    sum += number;
                }
                found_number = false;
                number = 0;
            }
        }
        if found_number {
            let end = line.len() - 1;
            if has_adjacent_symbol(&matrix, i, start, end) {
                sum += number;
            }
        }
    }
    sum
}

#[test]
fn day3_1() {
    let i = &parse(
        "467..114.
...*.....
..35..633
......#..
617*.....
.....+.58
..592....
......755
...$.*...
.664.598.",
    );
    assert_eq!(solve1(i), 4361);
    assert_eq!(solve2(i), 467835);
}
