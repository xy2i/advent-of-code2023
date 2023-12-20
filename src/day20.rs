use aoc_runner_derive::aoc;

use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Block<'a> {
    Broadcast(Vec<&'a str>),
    FlipFlop(bool, Vec<&'a str>),
    Conjunction(HashMap<&'a str, bool>, Vec<&'a str>),
}

impl<'a> Block<'a> {
    fn parse(s: &'a str) -> (&'a str, Self) {
        let (left, out) = s.split_once(" -> ").unwrap();
        let out: Vec<_> = out.split(", ").collect();
        if let Some(name) = left.strip_prefix('&') {
            (name, Block::Conjunction(HashMap::new(), out))
        } else if let Some(name) = left.strip_prefix('%') {
            (name, Block::FlipFlop(false, out))
        } else {
            (left, Block::Broadcast(out))
        }
    }

    fn parse_all(input: &'a str) -> HashMap<&'a str, Self> {
        let mut circuit: HashMap<&str, Block<'_>> = input.lines().map(Self::parse).collect();
        let mut inputs: HashMap<&str, HashMap<&str, bool>> = HashMap::new();
        for (k, v) in &circuit {
            let k: &&str = k;
            let out = match v {
                Block::Broadcast(o) | Block::FlipFlop(_, o) | Block::Conjunction(_, o) => o.clone(),
            };
            for o in out {
                inputs.entry(o).or_default().insert(*k, false);
            }
        }
        for (k, v) in &mut circuit {
            if let Block::Conjunction(mem, _) = v {
                *mem = inputs.remove(k).unwrap_or_default();
            }
        }
        circuit
    }

    fn run(&mut self, origin: &'a str, value: bool) -> (bool, &[&'a str]) {
        match self {
            Block::Broadcast(out) => (value, out),
            Block::FlipFlop(state, out) => {
                if value {
                    (false, &[])
                } else {
                    *state = !*state;
                    (*state, out)
                }
            }
            Block::Conjunction(mem, out) => {
                *mem.entry(origin).or_insert(false) = value;
                (!mem.values().all(|k| *k), out)
            }
        }
    }
}

#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
    let mut circuit = Block::parse_all(input);
    let mut signals = [0, 0];
    for _ in 0..1000 {
        let mut waiting = VecDeque::new();
        waiting.push_back(("button", "broadcaster", false));
        signals[0] += 1;
        while let Some((origin, target, value)) = waiting.pop_front() {
            if let Some(tgt) = circuit.get_mut(target) {
                let (s, out) = tgt.run(origin, value);
                signals[usize::from(s)] += out.len();
                for o in out {
                    waiting.push_back((target, o, s));
                }
            }
        }
    }
    signals[0] * signals[1]
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
    let mut circuit = Block::parse_all(input);
    let mut cycles: HashMap<&str, usize> = circuit
        .values()
        .find_map(|v| {
            if let Block::Conjunction(inp, out) = v {
                if out.contains(&"rx") {
                    return Some(inp.keys().map(|n| (*n, 0)));
                }
            }
            None
        })
        .unwrap()
        .collect();
    dbg!(&cycles);

    for i in 1.. {
        let mut waiting = VecDeque::new();
        waiting.push_back(("button", "broadcaster", false));
        while let Some((origin, target, value)) = waiting.pop_front() {
            if let Some(tgt) = circuit.get_mut(target) {
                let (s, out) = tgt.run(origin, value);
                for o in out {
                    waiting.push_back((target, o, s));
                    if !s && cycles.get(o) == Some(&0) {
                        cycles.insert(*o, i);
                        if cycles.values().all(|&v| v > 0) {
                            let mut lcm = 1;
                            for mut c in cycles.values().copied() {
                                let d = lcm * c;
                                while c != 0 {
                                    (lcm, c) = (c, lcm % c);
                                }
                                lcm = d / lcm;
                            }
                            return lcm;
                        }
                    }
                }
            }
        }
    }
    unreachable!()
}
