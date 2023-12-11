use aoc_runner_derive::aoc;

#[aoc(day11, part1)]
pub fn run(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut points = vec![];
    let mut empty_lines = vec![];
    let mut empty_columns = vec![];

    for (i, line) in grid.iter().enumerate() {
        for (j, elem) in line.iter().enumerate() {
            if elem == &'#' {
                points.push((i, j));
            }
        }
    }

    for (i, line) in grid.iter().enumerate() {
        if line.iter().all(|&x| x == '.') {
            empty_lines.push(i);
        }
    }

    for i in 0..grid[0].len() {
        let mut empty = true;
        for j in 0..grid.len() {
            if grid[j][i] != '.' {
                empty = false;
            }
        }
        if empty {
            empty_columns.push(i);
        }
    }

    let mut sum = 0;
    for (i, (x1, y1)) in points.iter().enumerate() {
        for (j, (x2, y2)) in points[i..].iter().enumerate() {
            let start_x = std::cmp::min(x1, x2);
            let start_y = std::cmp::min(y1, y2);
            let end_x = std::cmp::max(x1, x2);
            let end_y = std::cmp::max(y1, y2);

            let line_start = empty_lines
                .iter()
                .position(|x| x > start_x)
                .unwrap_or(empty_lines.len());
            let line_end = empty_lines
                .iter()
                .position(|x| x > end_x)
                .unwrap_or(empty_lines.len());
            let col_start = empty_columns
                .iter()
                .position(|x| x > start_y)
                .unwrap_or(empty_columns.len());
            let col_end = empty_columns
                .iter()
                .position(|x| x > end_y)
                .unwrap_or(empty_columns.len());

            sum += (end_y - start_y)
                + (end_x - start_x)
                + (col_end - col_start)
                + (line_end - line_start)
        }
    }
    sum as u64
}
#[aoc(day11, part2)]
pub fn run2(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut points = vec![];
    let mut empty_lines = vec![];
    let mut empty_columns = vec![];

    for (i, line) in grid.iter().enumerate() {
        for (j, elem) in line.iter().enumerate() {
            if elem == &'#' {
                points.push((i, j));
            }
        }
    }

    for (i, line) in grid.iter().enumerate() {
        if line.iter().all(|&x| x == '.') {
            empty_lines.push(i);
        }
    }

    for i in 0..grid[0].len() {
        let mut empty = true;
        for j in 0..grid.len() {
            if grid[j][i] != '.' {
                empty = false;
            }
        }
        if empty {
            empty_columns.push(i);
        }
    }

    let mut sum = 0;
    for (i, (x1, y1)) in points.iter().enumerate() {
        for (j, (x2, y2)) in points[i..].iter().enumerate() {
            let start_x = std::cmp::min(x1, x2);
            let start_y = std::cmp::min(y1, y2);
            let end_x = std::cmp::max(x1, x2);
            let end_y = std::cmp::max(y1, y2);

            let line_start = empty_lines
                .iter()
                .position(|x| x > start_x)
                .unwrap_or(empty_lines.len());
            let line_end = empty_lines
                .iter()
                .position(|x| x > end_x)
                .unwrap_or(empty_lines.len());
            let col_start = empty_columns
                .iter()
                .position(|x| x > start_y)
                .unwrap_or(empty_columns.len());
            let col_end = empty_columns
                .iter()
                .position(|x| x > end_y)
                .unwrap_or(empty_columns.len());

            sum += (end_y - start_y)
                + (end_x - start_x)
                + (col_end - col_start) * 999_999
                + (line_end - line_start) * 999_999
        }
    }
    sum as u64
}

#[test]
pub fn day11() {
    assert_eq!(
        run("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."),
        374
    );
}
