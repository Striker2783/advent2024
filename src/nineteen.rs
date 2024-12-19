use std::{fs, path::Path};

pub fn run(path: &Path) {
    let s = fs::read_to_string(path).unwrap();
    println!("{}", Solution::solve_one(&s));
    println!("{}", Solution::solve_two(&s));
}
struct Solution {
    patterns: Vec<String>,
    designs: Vec<String>,
}
impl Solution {
    fn solve_one(s: &str) -> u32 {
        let solution = Solution::from(s);
        solution
            .designs
            .iter()
            .filter(|d| solution.recurse_one(d))
            .count() as u32
    }
    fn recurse_one(&self, d: &str) -> bool {
        if d.is_empty() {
            return true;
        }
        for p in &self.patterns {
            if p.len() > d.len() || p != &d[..p.len()] {
                continue;
            }
            if self.recurse_one(&d[p.len()..]) {
                return true;
            }
        }
        false
    }
    fn solve_two(s: &str) -> u64 {
        let solution = Solution::from(s);
        solution
            .designs
            .iter()
            // .inspect(|b| println!("{b}"))
            .map(|d| solution.recurse_two(d, &mut vec![None; d.len()+1]))
            .sum()
    }
    fn recurse_two(&self, d: &str, memo: &mut [Option<u64>]) -> u64 {
        if let Some(n) = memo[d.len()] {
            return n;
        }
        if d.is_empty() {
            return 1;
        }
        let mut sum = 0;
        for p in &self.patterns {
            if p.len() > d.len() || p != &d[..p.len()] {
                continue;
            }
            sum += self.recurse_two(&d[p.len()..], memo);
        }
        memo[d.len()] = Some(sum);
        sum
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut split = value.split("\n\n");
        Self {
            patterns: split
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.to_string())
                .collect(),
            designs: split
                .next()
                .unwrap()
                .split('\n')
                .map(|s| s.to_string())
                .collect(),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::nineteen::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(
            Solution::solve_one(
                "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            ),
            6
        );
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(
            Solution::solve_two(
                "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            ),
            16
        );
    }
}
