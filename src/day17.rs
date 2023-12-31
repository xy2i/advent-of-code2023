use aoc_runner_derive::aoc;
use std::cmp::Reverse;

type Pt = (u8, u8);
type Grid = Vec<Vec<u8>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn apply(self, (row, col): Pt, grid: &Grid) -> Option<Pt> {
        match self {
            Dir::Up if row > 0 => Some((row - 1, col)),
            Dir::Down if row < grid.len() as u8 - 1 => Some((row + 1, col)),
            Dir::Left if col > 0 => Some((row, col - 1)),
            Dir::Right if col < grid[0].len() as u8 - 1 => Some((row, col + 1)),
            _ => None,
        }
    }

    fn turns(self) -> [Dir; 2] {
        match self {
            Dir::Right => [Dir::Up, Dir::Down],
            Dir::Up => [Dir::Left, Dir::Right],
            Dir::Left => [Dir::Up, Dir::Down],
            Dir::Down => [Dir::Left, Dir::Right],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Step {
    pt: Pt,
    dir: Dir,
}

impl Step {
    fn advance(self, grid: &Grid) -> Option<(u64, Step)> {
        self.dir.apply(self.pt, grid).map(|pt @ (row, col)| {
            (
                grid[row as usize][col as usize] as u64,
                Step { pt, dir: self.dir },
            )
        })
    }
}

fn neighbors(grid: &Grid, mut cur: Step, min: u8, max: u8) -> Vec<(u64, Step)> {
    let mut nbrs = Vec::new();
    let mut cost = 0;
    for _ in 0..min {
        if let Some((step_cost, step)) = cur.advance(grid) {
            cost += step_cost;
            cur = step;
        } else {
            return Vec::new();
        };
    }
    for _ in 0..max - min + 1 {
        for dir in cur.dir.turns() {
            nbrs.push((cost, Step { pt: cur.pt, dir }));
        }
        if let Some((step_cost, step)) = cur.advance(grid) {
            cost += step_cost;
            cur = step;
        } else {
            break;
        };
    }
    nbrs
}

fn min_path(grid: &Grid, start: Pt, end: Pt, min_steps: u8, max_steps: u8) -> Option<u64> {
    let mut costs: Vec<Vec<[u64; 4]>> = vec![vec![[u64::MAX; 4]; 256]; 256];
    let mut q = std::collections::BinaryHeap::new();
    for dir in [Dir::Right, Dir::Down] {
        let step = Step { pt: start, dir };
        q.push(Reverse((0, step)));
        costs[step.pt.0 as usize][step.pt.1 as usize][step.dir as usize] = 0;
    }
    while let Some(Reverse((cost, step))) = q.pop() {
        if step.pt == end {
            return Some(cost);
        }
        for (nbr_step_cost, nbr_step) in neighbors(grid, step, min_steps, max_steps) {
            let nbr_cost = cost + nbr_step_cost;
            if nbr_cost
                < costs[nbr_step.pt.0 as usize][nbr_step.pt.1 as usize][nbr_step.dir as usize]
            {
                q.push(Reverse((nbr_cost, nbr_step)));
                costs[nbr_step.pt.0 as usize][nbr_step.pt.1 as usize][nbr_step.dir as usize] =
                    nbr_cost;
            }
        }
    }
    None
}

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect()
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> u64 {
    let grid = parse(input);
    let start = (0, 0);
    let end = (grid.len() as u8 - 1, grid[0].len() as u8 - 1);
    min_path(&grid, start, end, 1, 3).unwrap()
}
#[aoc(day17, part2)]
pub fn part2(input: &str) -> u64 {
    let grid = parse(input);
    let start = (0, 0);
    let end = (grid.len() as u8 - 1, grid[0].len() as u8 - 1);
    min_path(&grid, start, end, 4, 10).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        assert_eq!(part1(input), 102);
        //assert_eq!(part2(input), 94);
    }
}
