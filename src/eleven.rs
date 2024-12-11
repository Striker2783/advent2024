use std::{collections::HashMap, fs, path::Path};

pub fn run(path: &Path) {
    let s = fs::read_to_string(path).unwrap();
    println!("{}", Solution::solve_one(&s));
    println!("{}", Solution::solve_two(&s));
}

struct Solution {
    v: Vec<u64>,
    map: HashMap<u64, u64>,
}
impl Solution {
    pub fn solve_one(s: &str) -> u64 {
        let mut s = Self::from(s);
        for _ in 0..25 {
            s.blink();
        }
        s.map.iter().fold(0, |acc, (_, v)| acc + *v)
    }
    pub fn solve_two(s: &str) -> u64 {
        let mut s = Self::from(s);
        for _ in 0..75 {
            s.blink();
        }
        s.map.iter().fold(0, |acc, (_, v)| acc + *v)
    }
    fn split(k: u64) -> Option<(u64, u64)> {
        let s = k.to_string();
        if s.len() % 2 == 0 {
            Some((
                s[0..(s.len() / 2)].parse().unwrap(),
                s[(s.len() / 2)..].parse().unwrap(),
            ))
        } else {
            None
        }
    }
    fn blink(&mut self) {
        let mut new_map = HashMap::new();
        for (&k, &v) in &self.map {
            if k == 0 {
                new_map.entry(1).and_modify(|n| *n += v).or_insert(v);
            } else if let Some((a, b)) = Self::split(k) {
                new_map.entry(a).and_modify(|n| *n += v).or_insert(v);
                new_map.entry(b).and_modify(|n| *n += v).or_insert(v);
            } else {
                new_map.entry(k * 2024).and_modify(|n| *n += v).or_insert(v);
            }
        }
        self.map = new_map;
    }
}

impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let v = value.split(' ').map(|s| s.parse().unwrap()).collect();
        let mut map = HashMap::new();
        for &n in &v {
            map.entry(n).and_modify(|v| *v += 1).or_insert(1u64);
        }
        Self { v, map }
    }
}
#[cfg(test)]
mod tests {
    use crate::eleven::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(Solution::solve_one("125 17"), 55312);
    }
}
