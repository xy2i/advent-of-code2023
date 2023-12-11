use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[aoc(day10, part1)]
pub fn run(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (s_x, s_y) = input
        .lines()
        .enumerate()
        .map(|(i, s)| (i, s, s.find('S')))
        .find(|(_, _, j)| j.is_some())
        .map(|(i, _, j)| (j.unwrap() as i32, i as i32))
        .unwrap();

    use Dir::*;
    [
        (s_x - 1, s_y, Left),
        (s_x, s_y + 1, Down),
        (s_x + 1, s_y, Right),
        (s_x, s_y - 1, Up),
    ]
    .iter()
    .filter(|&&(x, y, _)| {
        !(x < 0 || y < 0 || x as usize >= grid[0].len() || y as usize >= grid.len())
    })
    .map(|&pos| {
        std::iter::successors(Some(pos), |&(x, y, dir)| {
            if x < 0 || y < 0 || x as usize >= grid[0].len() || y as usize >= grid.len() {
                return None;
            }
            match (grid[y as usize][x as usize], dir) {
                ('|', Up) => Some((x, y - 1, Up)),
                ('|', Down) => Some((x, y + 1, Down)),
                ('-', Left) => Some((x - 1, y, Left)),
                ('-', Right) => Some((x + 1, y, Right)),
                ('L', Left) => Some((x, y - 1, Up)),
                ('L', Down) => Some((x + 1, y, Right)),
                ('J', Right) => Some((x, y - 1, Up)),
                ('J', Down) => Some((x - 1, y, Left)),
                ('7', Right) => Some((x, y + 1, Down)),
                ('7', Up) => Some((x - 1, y, Left)),
                ('F', Up) => Some((x + 1, y, Right)),
                ('F', Left) => Some((x, y + 1, Down)),
                _ => None,
            }
        })
        .position(|(x, y, _)| grid[y as usize][x as usize] == 'S')
        .map(|v| (v + 1) / 2)
        // Some(p), none
    })
    .flatten()
    .next()
    .unwrap() as u64
}

#[aoc(day10, part2)]
pub fn run2(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut dir = 'S';

    let mut start = (0, 0);

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == 'S' {
                start = (x, y);
                break;
            }
        }
    }
    let mut pos = start;
    // Check north of start
    if pos.0 > 0 && ['|', '7', 'F'].contains(&grid[pos.0 - 1][pos.1]) {
        dir = 'N';
        pos.0 -= 1;
    }
    // Check east of start
    else if pos.1 < grid[pos.0].len() - 1 && ['-', 'J', '7'].contains(&grid[pos.0][pos.1 + 1]) {
        dir = 'E';
        pos.1 += 1;
    }
    // Check south of start
    else if pos.0 < grid.len() - 1 && ['|', 'L', 'J'].contains(&grid[pos.0 + 1][pos.1]) {
        dir = 'S';
        pos.0 += 1;
    }

    let mut pipe2dir = std::collections::HashMap::new();
    pipe2dir.insert('|', ('N', 'S'));
    pipe2dir.insert('-', ('E', 'W'));
    pipe2dir.insert('L', ('N', 'E'));
    pipe2dir.insert('7', ('S', 'W'));
    pipe2dir.insert('J', ('N', 'W'));
    pipe2dir.insert('F', ('S', 'E'));

    let mut route = vec![];
    loop {
        route.push((pos.0 as f64, pos.1 as f64));

        if grid[pos.0][pos.1] == 'S' {
            break;
        }
        dir = match dir {
            'N' => 'S',
            'E' => 'W',
            'S' => 'N',
            'W' => 'E',
            _ => unreachable!(),
        };

        let next_dir = if pipe2dir.get(&grid[pos.0][pos.1]).unwrap().0 == dir {
            pipe2dir.get(&grid[pos.0][pos.1]).unwrap().1
        } else {
            pipe2dir.get(&grid[pos.0][pos.1]).unwrap().0
        };

        pos = match next_dir {
            'N' => (pos.0 - 1, pos.1),
            'E' => (pos.0, pos.1 + 1),
            'S' => (pos.0 + 1, pos.1),
            'W' => (pos.0, pos.1 - 1),
            _ => unreachable!(),
        };
        dir = next_dir;
    }

    let mut counter = 0;

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if !route.contains(&(x as f64, y as f64))
                && is_point_in_polygon((x as f64, y as f64), &route)
            {
                counter += 1;
            }
        }
    }

    counter
}

fn is_point_in_polygon(point: (f64, f64), polygon: &Vec<(f64, f64)>) -> bool {
    let mut is_inside = false;
    let mut j = polygon.len() - 1;

    for i in 0..polygon.len() {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        let intersect = ((yi > point.1) != (yj > point.1))
            && (point.0 < (xj - xi) * (point.1 - yi) / (yj - yi) + xi);

        if intersect {
            is_inside = !is_inside;
        }

        j = i;
    }

    is_inside
}

#[test]
fn day10() {
    assert_eq!(
        run(".....
.S-7.
.|.|.
.L-J.
....."),
        4
    );

    assert_eq!(
        run("-L|F7
7S-7|
L|7||
-L-J|
L|-JF"),
        4
    );

    assert_eq!(
        run("..F7.
.FJ|.
SJ.L7
|F--J
LJ..."),
        8
    );

    assert_eq!(
        run("7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"),
        8
    )
}
