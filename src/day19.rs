use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    pub fn new(s: &str) -> Self {
        let mut part = Self {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        let s = &s[1..s.len() - 1]; // { }

        for (i, kv) in s.split(',').enumerate() {
            let kv = &kv[2..]; // trim "x="
            let kv = kv.as_bytes();
            match i {
                0 => part.x = parse_num(kv),
                1 => part.m = parse_num(kv),
                2 => part.a = parse_num(kv),
                3 => part.s = parse_num(kv),
                _ => unreachable!(),
            }
        }
        part
    }

    pub fn rating(&self) -> u16 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Rule {
    LtX(u16),
    GtX(u16),
    LtM(u16),
    GtM(u16),
    LtA(u16),
    GtA(u16),
    LtS(u16),
    GtS(u16),
}

pub fn parse_num(v: &[u8]) -> u16 {
    let mut n = 0;
    for &b in v {
        if b.is_ascii_digit() {
            n = n * 10 + u16::from(b & 0xf);
        }
    }
    n
}

impl Rule {
    fn new(s: &str) -> Rule {
        use Rule::*;
        let n = parse_num(s[2..s.len()].as_bytes());
        let s = s.as_bytes();
        match (s[0], s[1]) {
            (b'x', b'<') => LtX(n),
            (b'x', b'>') => GtX(n),
            (b'm', b'<') => LtM(n),
            (b'm', b'>') => GtM(n),
            (b'a', b'<') => LtA(n),
            (b'a', b'>') => GtA(n),
            (b's', b'<') => LtS(n),
            (b's', b'>') => GtS(n),
            _ => unreachable!(),
        }
    }

    fn apply(self, part: &Part) -> bool {
        match self {
            Rule::LtX(n) => part.x < n,
            Rule::GtX(n) => part.x > n,
            Rule::LtM(n) => part.m < n,
            Rule::GtM(n) => part.m > n,
            Rule::LtA(n) => part.a < n,
            Rule::GtA(n) => part.a > n,
            Rule::LtS(n) => part.s < n,
            Rule::GtS(n) => part.s > n,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Workflow<'a> {
    rules: [Option<Rule>; 4],
    targets: [Option<&'a str>; 5],
}

impl<'a> Workflow<'a> {
    pub fn new(s: &'a str) -> Self {
        let mut workflow = Self {
            rules: [None; 4],
            targets: [None; 5],
        };

        for (i, entry) in s.split(',').enumerate() {
            let mid = entry.find(':');
            match mid {
                None => {
                    workflow.targets[i] = Some(entry);
                }
                Some(mid) => {
                    let (rule, target) = entry.split_at(mid);
                    let target = &target[1..];
                    workflow.rules[i] = Some(Rule::new(rule));
                    workflow.targets[i] = Some(target);
                }
            }
        }

        workflow
    }

    pub fn apply(&self, part: &Part) -> &'a str {
        for (i, rule) in self.rules.iter().enumerate() {
            if rule.is_none() {
                return self.targets[i].unwrap();
            }

            if self.rules[i].unwrap().apply(part) {
                return self.targets[i].unwrap();
            }
        }
        unreachable!()
    }
}

#[derive(Debug)]
struct System<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
}

impl<'a> System<'a> {
    /// Pass in whole string, will stop at space
    pub fn new(s: &mut impl Iterator<Item = &'a str>) -> Self {
        let mut workflows = HashMap::new();

        for line in s {
            if line.is_empty() {
                break;
            }

            let op_bracket = line.find('{').unwrap();
            let label = &line[..op_bracket];
            let workflow = &line[op_bracket + 1..line.len() - 1];

            workflows.insert(label, Workflow::new(workflow));
        }

        Self { workflows }
    }

    pub fn run(&self, part: &Part) -> bool {
        let mut current = "in";
        loop {
            current = self.workflows[current].apply(part);

            match current {
                "R" => return false,
                "A" => return true,
                _ => (),
            }
        }
    }
}

#[aoc(day19, part1)]
pub fn run(s: &str) -> u64 {
    let mut lines = s.lines();
    let system = System::new(&mut lines);

    lines
        .map(Part::new)
        .map(|p| (system.run(&p), p))
        //.inspect(|x| println!("{x:?}, {}", x.1.rating()))
        .filter(|&(b, _)| b)
        .map(|(_, p)| p.rating() as u64)
        .sum()
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
            19114
        );
    }
}
