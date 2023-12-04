use std::str::FromStr;

pub fn numbers_space<T: FromStr>(line: &str) -> impl Iterator<Item = T> + '_ {
    line.split_whitespace().filter_map(|s| s.parse::<T>().ok())
}
