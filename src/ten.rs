use std::{collections::HashSet, fs, path::Path};

pub fn run(path: &Path) {
    let s = fs::read_to_string(path).unwrap();
    println!("{}", Solution::solve_one(&s));
    println!("{}", Solution::solve_two(&s));
}
struct Solution {
    v: Vec<Vec<u8>>,
}
impl Solution {
    pub fn solve_one(s: &str) -> u32 {
        let solution = Solution::from(s);
        (0..solution.v.len())
            .map(|i| {
                (0..solution.v[i].len())
                    .filter_map(|j| {
                        if solution.v[i][j] != 0 {
                            None
                        } else {
                            let mut set = HashSet::new();
                            Some(solution.one_helper(&mut set, i, j, 0))
                        }
                    })
                    .sum::<u32>()
            })
            .sum()
    }    pub fn solve_two(s: &str) -> u32 {
        let solution = Solution::from(s);
        (0..solution.v.len())
            .map(|i| {
                (0..solution.v[i].len())
                    .filter_map(|j| {
                        if solution.v[i][j] != 0 {
                            None
                        } else {
                            Some(solution.two_helper(i, j, 0))
                        }
                    })
                    .sum::<u32>()
            })
            .sum()
    }
    fn one_helper(&self, set: &mut HashSet<(usize, usize)>, i: usize, j: usize, curr: u8) -> u32 {
        if set.contains(&(i, j)) {
            return 0;
        }
        let n = self.v[i][j];
        if n != curr {
            return 0;
        }
        set.insert((i, j));
        if n == 9 {
            return 1;
        }
        let mut sum = 0;
        if i < self.v.len() - 1 {
            sum += self.one_helper(set, i + 1, j, n + 1);
        }
        if i > 0 {
            sum += self.one_helper(set, i - 1, j, n + 1);
        }
        if j < self.v[i].len() - 1 {
            sum += self.one_helper(set, i, j + 1, n + 1);
        }
        if j > 0 {
            sum += self.one_helper(set, i, j - 1, n + 1);
        }
        sum
    }
    fn two_helper(&self, i: usize, j: usize, curr: u8) -> u32 {
        let n = self.v[i][j];
        if n != curr {
            return 0;
        }
        if n == 9 {
            return 1;
        }
        let mut sum = 0;
        if i < self.v.len() - 1 {
            sum += self.two_helper(i + 1, j, n + 1);
        }
        if i > 0 {
            sum += self.two_helper(i - 1, j, n + 1);
        }
        if j < self.v[i].len() - 1 {
            sum += self.two_helper(i, j + 1, n + 1);
        }
        if j > 0 {
            sum += self.two_helper(i, j - 1, n + 1);
        }
        sum
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        Self {
            v: value
                .lines()
                .map(|line| line.as_bytes().iter().map(|&b| b - b'0').collect())
                .collect(),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::ten::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(
            Solution::solve_one(
                "0123
1234
8765
9876
"
            ),
            1
        );
        assert_eq!(
            Solution::solve_one(
                "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"
            ),
            36
        );
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(Solution::solve_two("89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"), 81);
    }
}
