use std::{fs, path::Path};

pub fn run(f: &Path) {
    let s = fs::read_to_string(f).unwrap();
    println!("{}", Solution::solve_one(&s));
    println!("{}", Solution::solve_two(&s));
}

struct Solution {
    v: Vec<u32>,
    s: Vec<Option<u32>>,
}
impl Solution {
    pub fn solve_one(s: &str) -> u64 {
        let mut solution = Solution::from(s);
        solution.rearrange_s();
        solution
            .s
            .iter()
            .flatten()
            .cloned()
            .enumerate()
            .fold(0, |acc, (i, n)| acc + (i as u64) * (n as u64))
    }
    pub fn solve_two(s: &str) -> u64 {
        let mut solution = Solution::from(s);
        solution.rearrange_s_2();
        solution
            .s
            .iter()
            .cloned()
            .enumerate()
            .fold(0, |acc, (i, n)| match n {
                Some(n) => acc + (i as u64) * (n as u64),
                _ => acc,
            })
    }
    fn rearrange_s(&mut self) {
        let mut curr_idx = 0;
        for i in (0..self.s.len()).rev() {
            while self.s[curr_idx].is_some() {
                curr_idx += 1;
            }
            if curr_idx >= i {
                break;
            }
            self.s[curr_idx] = self.s[i];
            self.s[i] = None;
        }
    }
    fn rearrange_s_2(&mut self) {
        'outer: for i in (0..self.s.len()).rev() {
            if self.s[i].is_none() {
                continue;
            }
            let size = self.v[self.s[i].unwrap() as usize * 2];
            let mut curr_idx = 0;
            loop {
                while self.s[curr_idx].is_some() {
                    curr_idx += 1;
                    if curr_idx >= i || curr_idx >= self.s.len() {
                        continue 'outer;
                    }
                }
                let consecutive = (curr_idx..self.s.len())
                    .take_while(|&i| self.s[i].is_none())
                    .count();
                if consecutive as u32 >= size {
                    break;
                }
                curr_idx += consecutive;
            }
            let temp = self.s[i];
            for j in 0..(size as usize) {
                self.s[curr_idx + j] = temp;
                self.s[i - j] = None;
            }
        }
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let v: Vec<_> = value.bytes().map(|b| (b - b'0') as u32).collect();
        let mut s = vec![];
        for (i, n) in v.iter().cloned().enumerate() {
            for _ in 0..n {
                if i % 2 == 0 {
                    s.push(Some(i as u32 / 2));
                } else {
                    s.push(None);
                }
            }
        }
        Solution { v, s }
    }
}

#[cfg(test)]
mod tests {
    use crate::nine::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(Solution::solve_one("2333133121414131402"), 1928);
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(Solution::solve_two("2333133121414131402"), 2858);
    }
}
