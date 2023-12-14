use crate::transpose;
use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day14, part1)]
pub fn run(s: &str) -> u64 {
    let grid = transpose(
        s.lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let n = grid[0].len();

    grid.into_iter()
        .map(|col| {
            col.into_iter()
                .enumerate()
                .fold((n, 0), |(val, sum), (i, x)| {
                    if val == 0 {
                        return (val, sum);
                    }
                    match x {
                        '.' => (val, sum),
                        'O' => (val - 1, sum + val),
                        '#' => (n - i - 1, sum),
                        _ => unreachable!(),
                    }
                })
                .1
        })
        .sum::<usize>() as u64
}

fn rotate_r(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_v = vec![vec!['.'; v.len()]; v.len()];
    for (i, line) in v.iter().enumerate() {
        for (j, &e) in line.iter().enumerate() {
            new_v[j][v.len() - 1 - i] = e
        }
    }
    new_v
}

fn push_r(v: &mut Vec<Vec<char>>) {
    for line in v {
        let mut nb_rocks = 0;
        for (i, c) in line.clone().iter().enumerate() {
            match c {
                '.' => (),
                'O' => {
                    nb_rocks += 1;
                    line[i] = '.'
                }
                '#' => {
                    // backfill
                    for j in i - nb_rocks..i {
                        if line[j] == '#' {
                            break;
                        }
                        line[j] = 'O';
                    }
                    nb_rocks = 0;
                }
                _ => unreachable!(),
            }
        }
        // last backfill
        let i = line.len();
        for j in i - nb_rocks..i {
            if line[j] == '#' {
                break;
            }
            line[j] = 'O';
        }
    }
}

pub fn cycle(v: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut v = rotate_r(&v);
    push_r(&mut v);
    let mut v = rotate_r(&v);
    push_r(&mut v);
    let mut v = rotate_r(&v);
    push_r(&mut v);
    let mut v = rotate_r(&v);
    push_r(&mut v);
    v
}

/*fn p(v: &Vec<Vec<char>>) {
    for line in v {
        let mut s = String::new();
        for &ch in line {
            s.push(ch);
        }
        println!("{s}");
    }
    println!("========================== === === ===");
}*/

#[aoc(day14, part2)]
pub fn run2(s: &str) -> u64 {
    let mut grid = s
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut s = HashSet::new();
    loop {
        grid = cycle(&mut grid);
        if s.contains(&grid) {
            break;
        }
        s.insert(grid.clone());
    }
    let first_c = s.len();
    let mut clone = grid.clone();

    let mut s = HashSet::new();
    loop {
        grid = cycle(&mut grid);
        if s.contains(&grid) {
            break;
        }
        s.insert(grid.clone());
    }
    let c = s.len();

    let rest = (1_000_000_000 - first_c) % (c) - 1;

    for _ in 0..rest {
        clone = cycle(&mut clone);
    }

    let n = clone[0].len();
    clone
        .into_iter()
        .enumerate()
        .map(|(i, l)| l.into_iter().filter(|&c| c == 'O').count() * (n - i))
        .sum::<usize>() as u64
}

#[test]
pub fn day14() {
    assert_eq!(
        run("OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."),
        136
    );
    assert_eq!(
        run2(
            "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
        ),
        64
    )
}
