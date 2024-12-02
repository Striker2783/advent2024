use std::{
    cmp::Ordering,
    fs::{self},
    path::Path,
};

pub fn run(f: &Path) {
    let s = fs::read_to_string(f).unwrap();
    println!("{}", Solution::solve_one(&s));
    println!("{}", Solution::solve_two(&s));
}

struct Solution {
    v: Vec<Vec<u32>>,
}
impl Solution {
    fn solve_one(s: &str) -> u32 {
        let solution = Solution::from(s);
        solution
            .v
            .iter()
            .filter(|&v| {
                let (mut decrease, mut increase) = (true, true);
                for (i, &n) in v.iter().enumerate().skip(1) {
                    decrease = decrease && n < v[i - 1];
                    increase = increase && n > v[i - 1];
                    if n.abs_diff(v[i - 1]) > 3 {
                        return false;
                    }
                }
                increase || decrease
            })
            .count() as u32
    }
    fn solve_two(s: &str) -> u32 {
        let solution = Solution::from(s);
        solution
            .v
            .iter()
            .filter(|&v| {
                let helper = |c: Ordering| {
                    let mut skipped = None;
                    let mut skips = 1;
                    let mut skip_one = false;
                    for (mut i, n) in v.iter().cloned().enumerate().skip(1) {
                        if skipped.is_some() && skipped.unwrap() == i - 1 {
                            i = i - 1;
                        }
                        if n.cmp(&v[i - 1]) == c && n.abs_diff(v[i - 1]) <= 3 {
                            continue;
                        } else if skip_one && i == 1 && n.cmp(&v[1]) == c && n.abs_diff(v[1]) <= 3{
                            continue;
                        }
                        else if skips == 0 {
                            return false;
                        } else {
                            skips -= 1;
                            skipped = Some(i);
                            if i == 1 {
                                skip_one = true;
                            }
                        }
                    }
                    true
                };
                helper(Ordering::Greater) || helper(Ordering::Less)
            })
            .count() as u32
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut v = Vec::new();
        for (i, line) in value.lines().enumerate() {
            v.push(vec![]);
            for n in line.split(' ') {
                v[i].push(n.parse().unwrap());
            }
        }
        Solution { v }
    }
}

#[cfg(test)]
mod tests {
    use crate::two::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(
            Solution::solve_one("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9"),
            2
        );
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(Solution::solve_two("1 18 21 24 25 27"), 1);
        assert_eq!(
            Solution::solve_two("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9"),
            4
        );
        assert_eq!(Solution::solve_two("18 42 21 24 27"), 1);
        assert_eq!(Solution::solve_two("27 20 24 21 18"), 1);
        assert_eq!(Solution::solve_two("3 2 5 6 7"), 1);
    }
}
