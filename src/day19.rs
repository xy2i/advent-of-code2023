use aoc_runner_derive::aoc;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Eq, PartialEq)]
enum Condition<'a> {
    Always(&'a str),
    Ltgt(usize, bool, u32, &'a str),
}

impl<'a> Condition<'a> {
    fn split_part(&self, part: Part) -> (&'a str, Option<Part>, Option<Part>) {
        match *self {
            Condition::Always(dest) => (dest, Some(part), None),
            Condition::Ltgt(p, gt, v, dest) => {
                let relevant_range = part[p];

                let (matching, remaining) = split_range(relevant_range, gt, v);

                let mut matching_part = None;
                let mut remaining_part = None;

                if let Some(matching) = matching {
                    let mut new = part.clone();
                    new[p] = matching;
                    matching_part = Some(new);
                }
                if let Some(remaining) = remaining {
                    let mut new = part.clone();
                    new[p] = remaining;
                    remaining_part = Some(new);
                }

                (dest, matching_part, remaining_part)
            }
        }
    }
}

#[aoc(day19, part1)]
pub fn run(s: &str) -> usize {
    let (workflows, _items) = s.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|l| {
            let l: &str = &l[..l.len() - 1];
            let (name, l) = l.split_once('{').unwrap();
            let conds = l
                .split(',')
                .map(|c| {
                    if let Some((cond, dest)) = c.split_once(':') {
                        let prop = match cond.chars().next().unwrap() {
                            'x' => 0,
                            'm' => 1,
                            'a' => 2,
                            's' => 3,
                            _ => unreachable!(),
                        };
                        let gtlt = cond.chars().nth(1).unwrap() == '>';
                        let value: u32 = cond[2..].parse().unwrap();

                        Condition::Ltgt(prop, gtlt, value, dest)
                    } else {
                        Condition::Always(c)
                    }
                })
                .collect::<Vec<_>>();
            (name, conds)
        })
        .collect::<HashMap<_, _>>();

    let mut accepted = vec![];
    let start: Part = [(1, 4000), (1, 4000), (1, 4000), (1, 4000)];

    let mut q = VecDeque::new();
    q.push_front(("in", start));

    while let Some((label, mut part)) = q.pop_front() {
        let workflow = &workflows[label];

        for rule in workflow {
            let (next_label, matching, non_matching) = rule.split_part(part);

            if let Some(matching) = matching {
                match next_label {
                    "A" => {
                        accepted.push(matching);
                    }
                    "R" => (), // skip,
                    _ => q.push_front((next_label, matching)),
                }
            }

            if let Some(non_matching) = non_matching {
                part = non_matching;
                continue;
            } else {
                break;
            }
        }
    }

    accepted
        .iter()
        .map(|list| {
            list.iter()
                .map(|(start, end)| (end + 1 - start) as usize)
                .product::<usize>()
        })
        .sum()
}

type Part = [(u32, u32); 4];

fn split_range(range: (u32, u32), gt: bool, v: u32) -> (Option<(u32, u32)>, Option<(u32, u32)>) {
    if gt {
        if range.0 > v {
            (Some(range), None)
        } else if range.1 < v {
            (None, Some(range))
        } else {
            (Some((v + 1, range.1)), Some((range.0, v)))
        }
    } else {
        if range.1 < v {
            (Some(range), None)
        } else if range.0 > v {
            (None, Some(range))
        } else {
            (Some((range.0, v - 1)), Some((v, range.1)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        assert_eq!(
            run("px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"),
            167409079868000
        );
    }
}
