use aoc_runner_derive::aoc;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn step((x, y, dir): (i32, i32, Dir)) -> (i32, i32, Dir) {
    use Dir::*;
    match &dir {
        Up => (x, y - 1, dir),
        Down => (x, y + 1, dir),
        Left => (x - 1, y, dir),
        Right => (x + 1, y, dir),
    }
}

/*
pub fn p(grid: &Vec<Vec<char>>, pos: &(i32, i32), visited: &HashSet<(i32, i32)>) {
    for (y, l) in grid.iter().enumerate() {
        for (x, _c) in l.iter().enumerate() {
            if (x as i32, y as i32) == *pos {
                print!("O")
            } else if visited.contains(&(x as i32, y as i32)) {
                print!("#")
            } else {
                print!("{}", grid[y as usize][x as usize])
            }
        }
        println!("");
    }
}
*/
pub fn beam(s: &str, start: (i32, i32, Dir)) -> u64 {
    use Dir::*;
    let grid = s
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let nb_lines = grid.len() as i32;
    let nb_cols = grid[0].len() as i32;

    let mut visited_mirrors = HashSet::new();
    let mut visited_pos = HashSet::new();
    let mut mirror_dirs = HashMap::new();
    let mut positions = vec![start];

    while !positions.is_empty() {
        let (mut x, mut y, mut dir) = positions.pop().unwrap();

        while x >= 0 && x < nb_cols && y >= 0 && y < nb_lines {
            let pos = grid[y as usize][x as usize];
            visited_pos.insert((x, y));

            //println!("{x}:{y} {dir:?} {pos}, {visited}");
            //p(&grid, &(x, y), &visited_pos);
            //println!("");
            if visited_mirrors.contains(&(x, y)) {
                let &(vx, vy) = visited_pos.get(&(x, y)).unwrap();
                if matches!(grid[vy as usize][vx as usize], '/' | '\\') {
                    let sides = mirror_dirs
                        .entry((vx, vy))
                        .or_insert((false, false, false, false));

                    match dir {
                        Up => sides.0 = true,
                        Down => sides.1 = true,
                        Left => sides.2 = true,
                        Right => sides.3 = true,
                    }

                    if sides.0 && sides.1 && sides.2 && sides.3 {
                        break;
                    }
                } else {
                    break;
                }
            }
            if matches!(pos, '|' | '-' | '/' | '\\') {
                visited_mirrors.insert((x, y));
            }

            (x, y, dir) = match pos {
                '.' => step((x, y, dir)),
                '|' => match dir {
                    Left | Right => {
                        visited_mirrors.insert((x, y));
                        positions.push((x, y - 1, Up));
                        (x, y + 1, Down)
                    }
                    _ => step((x, y, dir)),
                },
                '-' => match dir {
                    Up | Down => {
                        visited_mirrors.insert((x, y));
                        positions.push((x - 1, y, Left));
                        (x + 1, y, Right)
                    }
                    _ => step((x, y, dir)),
                },
                '/' => match dir {
                    Right => (x, y - 1, Up),
                    Up => (x + 1, y, Right),
                    Left => (x, y + 1, Down),
                    Down => (x - 1, y, Left),
                },
                '\\' => match dir {
                    Left => (x, y - 1, Up),
                    Down => (x + 1, y, Right),
                    Right => (x, y + 1, Down),
                    Up => (x - 1, y, Left),
                },
                _ => unreachable!(),
            };
        }
    }

    visited_pos.len().try_into().unwrap()
}

use Dir::*;
#[aoc(day16, part1)]
pub fn run(s: &str) -> u64 {
    beam(s, (0, 0, Right))
}
#[aoc(day16, part2)]
pub fn run2(s: &str) -> u64 {
    let mut best = 0;

    let grid = s
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let nb_lines = grid.len() as i32;
    let nb_cols = grid[0].len() as i32;

    for x in 0..nb_cols {
        best = std::cmp::max(best, beam(s, (x, 0, Down)));
        best = std::cmp::max(best, beam(s, (x, nb_lines - 1, Up)));
    }
    for y in 0..nb_lines {
        best = std::cmp::max(best, beam(s, (0, y, Right)));
        best = std::cmp::max(best, beam(s, (nb_cols - 1, y, Left)));
    }

    best
}

#[test]
fn day16() {
    assert_eq!(
        run(r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#),
        46
    );
    assert_eq!(
        run2(
            r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#
        ),
        51
    );
}
