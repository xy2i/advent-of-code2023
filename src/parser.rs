use std::str::FromStr;

pub fn numbers_space<T: FromStr>(line: &str) -> impl Iterator<Item = T> + '_ {
    line.split_whitespace().filter_map(|s| s.parse::<T>().ok())
}

pub struct GetInts<'a>(&'a [u8]);

pub fn get_ints(bytes: &[u8]) -> GetInts<'_> {
    GetInts(bytes)
}

impl Iterator for GetInts<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        let mut shave = self.0.len();
        let mut n = 0u64;

        'outer: for (out_idx, b) in self.0.iter().enumerate() {
            if b.is_ascii_digit() {
                n = u64::from(b - b'0');

                for (idx, b) in self.0[out_idx + 1..].iter().enumerate() {
                    if !b.is_ascii_digit() {
                        shave = out_idx + idx + 1;
                        break 'outer;
                    }
                    n = n * 10 + u64::from(b - b'0');
                }

                break;
            }
        }

        if shave <= self.0.len() {
            self.0 = &self.0[shave..];
            Some(n)
        } else {
            None
        }
    }
}

pub fn parse_num(v: &[u8]) -> u64 {
    let mut n = 0;
    for &b in v {
        if b.is_ascii_digit() {
            n = n * 10 + u64::from(b & 0xf);
        }
    }

    n
}
