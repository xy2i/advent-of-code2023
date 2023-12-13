use aoc_runner_derive::aoc;
use cached::proc_macro::cached;

fn parse(input: &str) -> Vec<(&[u8], Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let (s, n) = l.split_once(" ").unwrap();
            (
                s.as_bytes(),
                n.split(",").map(|n| n.parse().unwrap()).collect::<Vec<_>>(),
            )
        })
        .collect()
}

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|(s, ns)| solve(s, None, &ns))
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> usize {
    let new_input = input.lines().fold(String::new(), |mut acc, l| {
        let (s, n) = l.split_once(" ").unwrap();
        acc.push_str(&format!("{s}?{s}?{s}?{s}?{s} {n},{n},{n},{n},{n}\n"));
        acc
    });
    part1(&new_input)
}

#[cached(
    key = "String",
    convert = r#"{format!("{:?}{:?}{:?}", s, in_group, cons)}"#
)]
fn solve(s: &[u8], in_group: Option<usize>, cons: &[usize]) -> usize {
    if s.is_empty() {
        return match in_group {
            Some(n) if cons == &[n] => 1,
            None if cons.is_empty() => 1,
            _ => 0,
        };
    }
    match (s[0], in_group, cons) {
        (b'.', None, _) | (b'?', None, []) => solve(&s[1..], None, cons),
        (b'.' | b'?', Some(n), [e, ..]) if n == *e => solve(&s[1..], None, &cons[1..]),
        (b'#' | b'?', Some(n), [e, ..]) if n < *e => solve(&s[1..], Some(n + 1), cons),
        (b'#', None, [_, ..]) => solve(&s[1..], Some(1), cons),
        (b'?', None, _) => solve(&s[1..], None, cons) + solve(&s[1..], Some(1), cons),
        _ => 0,
    }
}
