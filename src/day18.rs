use aoc_runner_derive::aoc;

#[aoc(day18, part1)]
pub fn run(s: &str) -> u32 {
    let instrs = s.lines().map(|l| {
        let mut chars = l.chars();
        let dir = chars.next().unwrap();
        chars.next().unwrap();
        let steps = chars.next().unwrap() as u8 - b'0';
        let next_step = chars.next().unwrap();
        let steps = if next_step.is_ascii_digit() {
            10 * steps + next_step as u8 - b'0'
        } else {
            steps
        };
        let steps = steps as i32;
        (dir, steps)
    });

    let (last, mut area, perimeter) = instrs.fold(
        ((0, 0), 0i32, 0i32),
        |((x0, y0), acc, count), (dir, steps)| {
            let (x1, y1) = match dir {
                'R' => (x0 + steps, y0),
                'D' => (x0, y0 + steps),
                'L' => (x0 - steps, y0),
                'U' => (x0, y0 - steps),
                _ => unreachable!(),
            };

            ((x1, y1), acc + (y0 + y1) * (x0 - x1), count + steps)
        },
    );
    area += last.1 * last.0;
    area /= 2;
    area.unsigned_abs() + 1 + perimeter as u32 / 2
}

#[aoc(day18, part2)]
pub fn run2(s: &str) -> u64 {
    let instrs = s.lines().map(|l| {
        let steps = &l[l.len() - 7..l.len() - 2];
        let dir = l[l.len() - 2..l.len() - 1].chars().next().unwrap();
        (dir, i64::from_str_radix(steps, 16).unwrap())
    });

    let (last, mut area, perimeter) = instrs.fold(
        ((0, 0), 0i64, 0i64),
        |((x0, y0), acc, count), (dir, steps)| {
            let (x1, y1) = match dir {
                '0' => (x0 + steps, y0),
                '1' => (x0, y0 + steps),
                '2' => (x0 - steps, y0),
                '3' => (x0, y0 - steps),
                _ => unreachable!(),
            };

            ((x1, y1), acc + (y0 + y1) * (x0 - x1), count + steps)
        },
    );
    area += last.1 * last.0;
    area /= 2;
    area.unsigned_abs() + 1 + perimeter as u64 / 2
}

#[test]
fn day18() {
    assert_eq!(
        run("R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"),
        62
    );
    assert_eq!(
        run2(
            "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
        ),
        952408144115
    );
}
