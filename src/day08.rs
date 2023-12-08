#[derive(Debug, Copy, Clone)]
struct Link {
    left: u16,
    right: u16,
}

#[derive(Debug)]
struct Solution<'a> {
    path: &'a str,
    tree: &'a [Link],
    aaa: u16,
    zzz: u16,
}

impl Solution<'_> {
    pub fn solve(&self) -> u64 {
        let mut pos = self.aaa as usize;
        let mut steps = 0;

        for c in self.path.chars().cycle() {
            pos = match c {
                'L' => self.tree[pos].left,
                'R' => self.tree[pos].right,
                _ => unreachable!(),
            } as usize;

            steps += 1;
            if pos == self.zzz as usize {
                break;
            };
        }

        steps
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    source: Id,
    left: Id,
    right: Id,
}
impl Line {
    pub fn parse(line: &[u8]) -> Self {
        Self {
            source: Id::new(&line[0..3]),
            left: Id::new(&line[7..10]),
            right: Id::new(&line[12..15]),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Id(u16);
impl Id {
    pub const fn new(s: &[u8]) -> Self {
        Self((((s[0] - b'A') as u16) << 10) + (((s[1] - b'A') as u16) << 5) + (s[2] - b'A') as u16)
    }
}

//use aoc_runner_derive::aoc;
//#[aoc(day8, part1)]
pub fn run(input: &str) -> u64 {
    let (path, input) = input.split_at(input.find('\n').unwrap());
    let input = &input[2..];

    const TREE_SIZE: usize = 1_000;
    const SENTINEL: u16 = (TREE_SIZE + 1) as u16;
    const AAA: Id = Id::new(b"AAA");
    const ZZZ: Id = Id::new(b"ZZZ");
    let mut lut = [SENTINEL; Id::new(b"ZZZ").0 as usize + 1];
    let mut tree = [Link { left: 0, right: 0 }; TREE_SIZE];

    let mut aaa = 0;
    let mut zzz = 0;

    let mut free: u16 = 0;
    for line in input.as_bytes().chunks(17) {
        let line = Line::parse(line);

        let source_pos = if lut[line.source.0 as usize] == SENTINEL {
            let pos = free;
            free += 1;
            lut[line.source.0 as usize] = pos;
            pos
        } else {
            lut[line.source.0 as usize]
        };
        let left_pos = if lut[line.left.0 as usize] == SENTINEL {
            let pos = free;
            free += 1;
            lut[line.left.0 as usize] = pos;
            pos
        } else {
            lut[line.left.0 as usize]
        };
        let right_pos = if lut[line.right.0 as usize] == SENTINEL {
            let pos = free;
            free += 1;
            lut[line.right.0 as usize] = pos;
            pos
        } else {
            lut[line.right.0 as usize]
        };

        tree[source_pos as usize] = Link {
            left: left_pos,
            right: right_pos,
        };

        match line.source {
            AAA => aaa = source_pos,
            ZZZ => zzz = source_pos,
            _ => (),
        }
    }

    let sol = Solution {
        aaa,
        zzz,
        path,
        tree: &tree[..free as usize],
    };

    sol.solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let i = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(run(i), 6);
    }
    #[test]
    fn test2() {
        let i = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(run(i), 2);
    }
}
