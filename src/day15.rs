use aoc_runner_derive::aoc;
use std::collections::VecDeque;

pub fn hash(s: &str) -> u8 {
    s.chars().fold(0, |cur, c| ((cur + c as u16) * 17) % 256) as u8
}

#[aoc(day15, part1)]
pub fn run(s: &str) -> u64 {
    s.split(',').map(hash).map(|x| x as u64).sum::<u64>()
}

#[aoc(day15, part2)]
pub fn run2(s: &str) -> u64 {
    let mut lut = vec![VecDeque::new(); 256];

    for seq in s.split(',') {
        let (label, inst) = if seq.chars().last().unwrap() == '-' {
            seq.split_at(seq.len() - 1)
        } else {
            seq.split_at(seq.len() - 2)
        };

        let b = &mut lut[hash(label) as usize];

        if inst == "-" {
            match b.iter().enumerate().find(|(_, (l, _))| *l == label) {
                Some((i, _)) => {
                    b.remove(i);
                }
                _ => {}
            }
        } else {
            let strength = inst.chars().nth(1).unwrap() as u8 - b'0';
            match b.iter().enumerate().find(|(_, (l, _))| *l == label) {
                Some((i, _)) => {
                    b[i] = (label, strength);
                }
                None => {
                    b.push_back((label, strength));
                }
            }
        }
    }

    lut.into_iter()
        .enumerate()
        .map(|(i, b)| {
            b.into_iter()
                .enumerate()
                .map(|(j, (_label, strength))| (i + 1) * (j + 1) * strength as usize)
                .sum::<usize>() as u64
        })
        .sum()
}

#[test]
fn day15() {
    assert_eq!(
        run("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        1320
    );
    assert_eq!(
        run2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        145
    );
}
