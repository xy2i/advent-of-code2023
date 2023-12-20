use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

use aoc_runner_derive::aoc;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Mod {
    Bcast,
    Flop(bool),      // %
    Conj(Vec<bool>), // &
}
impl Mod {
    pub fn conj(&mut self) -> &mut Vec<bool> {
        match self {
            Self::Conj(c) => c,
            _ => unreachable!(),
        }
    }
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[aoc(day20, part1)]
pub fn run(s: &str) -> u64 {
    // conj_mod -> [in, in, in] to conj
    //let conj_inputs = HashMap::new();

    let mut state = vec![];
    let mut mapping = HashMap::new();
    // reverse mapping
    let mut conj_mapping = HashMap::new();
    let mut labels = HashMap::new();

    // labels to indexes
    for (src_i, l) in s.lines().enumerate() {
        let (src, _) = l.split_once("->").unwrap();
        let src = src.trim();

        let (src, modu) = if src == "broadcaster" {
            (src, Mod::Bcast)
        } else {
            let (modu_ty, label) = src.split_at(1);
            let modu = match modu_ty {
                "%" => Mod::Flop(false),
                "&" => Mod::Conj(vec![]),
                _ => unreachable!(),
            };
            (label, modu)
        };

        state.push(modu);
        labels.insert(src, src_i);
    }

    let mut bcast_i = 0;
    for (src_i, l) in s.lines().enumerate() {
        let (src, dst) = l.split_once("->").unwrap();
        let src = src.trim();
        let dst = dst.trim();

        let src = if src == "broadcaster" {
            bcast_i = src_i;
            src
        } else {
            let (_, label) = src.split_at(1);
            label
        };

        let dst_list = dst
            .split(", ")
            .map(|label| *labels.get(&label).unwrap_or(&usize::MAX))
            .collect::<Vec<_>>();

        mapping.insert(src_i, dst_list.clone());

        for dst_i in dst_list {
            match state.get_mut(dst_i) {
                Some(Mod::Conj(v)) => {
                    let conj_i = v.len();
                    v.push(false);
                    conj_mapping.insert((src_i, dst_i), conj_i);
                }
                _ => (),
            }
        }
    }

    // simulate cycle
    let mut cycle: Vec<(u64, u64)> = vec![];
    let mut state_space = HashSet::new();
    let (mut cnt_lo, mut cnt_hi) = (0, 0);
    while state_space.insert(hash(&state)) {
        // low pulse = false, high = true
        let mut q: VecDeque<_> = [(bcast_i, false)].into_iter().collect();

        while let Some((src_i, pulse)) = q.pop_front() {
            //dbg!(src_i, &mapping.get(&src_i), pulse, &state.get(src_i));
            if pulse {
                cnt_hi += 1;
            } else {
                cnt_lo += 1;
            }

            if !mapping.contains_key(&src_i) {
                // it's joever
                continue;
            }

            let mut process_conj = vec![];

            match &mut state[src_i] {
                Mod::Bcast => {
                    for &dst_i in mapping[&src_i].iter() {
                        q.push_back((dst_i, false));
                        if conj_mapping.contains_key(&(src_i, dst_i)) {
                            process_conj.push((src_i, dst_i, true));
                        }
                    }
                }
                Mod::Flop(flipflop) => {
                    //println!("p:{pulse}, f:{flipflop}");
                    if !pulse {
                        *flipflop = !*flipflop;
                        //println!("sending {flipflop}");
                        for &dst_i in mapping[&src_i].iter() {
                            q.push_back((dst_i, *flipflop));
                            if conj_mapping.contains_key(&(src_i, dst_i)) {
                                process_conj.push((src_i, dst_i, *flipflop));
                            }
                        }
                    }
                }
                Mod::Conj(inputs) => {
                    let output = !inputs.iter().all(|&b| b);
                    //println!("conj sending {output}");
                    for &dst_i in mapping[&src_i].iter() {
                        q.push_back((dst_i, output));
                        if conj_mapping.contains_key(&(src_i, dst_i)) {
                            process_conj.push((src_i, dst_i, output));
                        }
                    }
                }
            }

            for (src_i, dst_i, pulse) in process_conj {
                let conj_i = conj_mapping[&(src_i, dst_i)];
                state[dst_i].conj()[conj_i] = pulse;
            }
        }
        dbg!(&cycle);
    }

    //dbg!(&cycle);

    let lo_cnt = cycle.last().unwrap().0 * (1000 / cycle.len() as u64)
        + cycle[1_000 % cycle.len()].0 as u64 * (1000 % cycle.len() as u64);
    let hi_cnt = cycle.last().unwrap().1 * (1000 / cycle.len() as u64)
        + cycle[1_000 % cycle.len()].1 as u64 * (1000 % cycle.len() as u64);

    lo_cnt * hi_cnt
}

#[test]
pub fn day20() {
    assert_eq!(
        run("broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"),
        32000000
    );
    assert_eq!(
        run("broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"),
        11687500
    )
}
