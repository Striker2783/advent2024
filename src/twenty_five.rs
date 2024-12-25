use std::{fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    println!("{}", Solution::solve_one(&s));
}
#[derive(Default)]
struct Solution {
    keys: Vec<Vec<u32>>,
    locks: Vec<Vec<u32>>,
}
impl Solution {
    fn solve_one(s: &str) -> u32 {
        let s = Self::from(s);
        let mut count = 0;
        for lock in &s.locks {
            'outer: for key in &s.keys {
                if lock.len() != key.len() {
                    continue;
                }
                for i in 0..lock.len() {
                    if lock[i] + key[i] > 5 {
                        continue 'outer;
                    }
                }
                count += 1;
            }
        }
        count
    }
    fn parse_lock(&mut self, s: &str) {
        let mut v = vec![0; 5];
        for (i, n) in s.lines().enumerate() {
            for (j, b) in n.bytes().enumerate() {
                if b == b'#' {
                    v[j] = i as u32;
                }
            }
        }
        self.locks.push(v);
    }
    fn parse_key(&mut self, s: &str) {
        let mut v = vec![0; 5];
        for (i, n) in s.lines().rev().enumerate() {
            for (j, b) in n.bytes().enumerate() {
                if b == b'#' {
                    v[j] = i as u32;
                }
            }
        }
        self.keys.push(v);
    }
    fn parse_key_or_lock(&mut self, s: &str) {
        if s.lines().next().unwrap().bytes().all(|b| b == b'#') {
            self.parse_lock(s);
        } else {
            self.parse_key(s);
        }
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut solution = Self::default();
        for k in value.split("\n\n") {
            solution.parse_key_or_lock(k);
        }
        solution
    }
}
#[cfg(test)]
mod tests {
    use crate::twenty_five::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(Solution::solve_one("#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"), 3);
    }
}