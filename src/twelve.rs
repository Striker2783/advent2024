use std::{
    fs,
    path::Path,
};

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
        let mut visited = vec![vec![false; solution.v[0].len()]; solution.v.len()];
        let result = (0..solution.v.len())
            .map(|i| {
                (0..solution.v[i].len())
                    .map(|j| {
                        let temp = solution.helper(&mut visited, (i, j), solution.v[i][j]);
                        temp.0 * temp.1
                    })
                    .sum::<u32>()
            })
            .sum::<u32>();
        result
    }
    pub fn solve_two(s: &str) -> u32 {
        let solution = Solution::from(s);
        let mut visited = vec![vec![false; solution.v[0].len()]; solution.v.len()];
        let result = (0..solution.v.len())
            .map(|i| {
                (0..solution.v[i].len())
                    .map(|j| {
                        let temp = solution.helper_2(&mut visited, (i, j), solution.v[i][j]);
                        temp.0 * temp.1
                    })
                    .sum::<u32>()
            })
            .sum::<u32>();
        result
    }
    fn helper(&self, visited: &mut Vec<Vec<bool>>, (i, j): (usize, usize), c: u8) -> (u32, u32) {
        if self.v[i][j] != c {
            return (0, 1);
        }
        if visited[i][j] {
            return (0, 0);
        }
        visited[i][j] = true;
        let mut sum = (1, 0);
        if j > 0 {
            let temp = self.helper(visited, (i, j - 1), c);
            sum = (sum.0 + temp.0, sum.1 + temp.1);
        } else {
            sum.1 += 1;
        }
        if i > 0 {
            let temp = self.helper(visited, (i - 1, j), c);
            sum = (sum.0 + temp.0, sum.1 + temp.1);
        } else {
            sum.1 += 1;
        }
        if j < self.v[i].len() - 1 {
            let temp = self.helper(visited, (i, j + 1), c);
            sum = (sum.0 + temp.0, sum.1 + temp.1);
        } else {
            sum.1 += 1;
        }
        if i < self.v.len() - 1 {
            let temp = self.helper(visited, (i + 1, j), c);
            sum = (sum.0 + temp.0, sum.1 + temp.1);
        } else {
            sum.1 += 1;
        }
        sum
    }
    fn helper_2(&self, visited: &mut Vec<Vec<bool>>, (i, j): (usize, usize), c: u8) -> (u32, u32) {
        if self.v[i][j] != c {
            return (0, 0);
        }
        if visited[i][j] {
            return (0, 0);
        }
        visited[i][j] = true;
        let mut sum = (1, 0);
        let mut arr = [false, false, false, false];
        if j > 0 {
            arr[0] = self.v[i][j - 1] == c;
            let temp = self.helper_2(visited, (i, j - 1), c);
            sum = (sum.0 + temp.0, sum.1 + temp.1);
        }
        if i > 0 {
            arr[1] = self.v[i - 1][j] == c;
            let temp = self.helper_2(visited, (i - 1, j), c);
            sum = (sum.0 + temp.0, sum.1 + temp.1);
        }
        if j < self.v[i].len() - 1 {
            arr[2] = self.v[i][j + 1] == c;
            let temp = self.helper_2(visited, (i, j + 1), c);
            sum = (sum.0 + temp.0, sum.1 + temp.1);
        }
        if i < self.v.len() - 1 {
            arr[3] = self.v[i + 1][j] == c;
            let temp = self.helper_2(visited, (i + 1, j), c);
            sum = (sum.0 + temp.0, sum.1 + temp.1);
        }
        for (i, &b) in arr.iter().enumerate() {
            if !b && !arr[(i + 1) % arr.len()] {
                sum.1 += 1;
            }
        }
        if arr[0] && arr[1] && self.v[i - 1][j - 1] != c {
            sum.1 += 1;
        }
        if arr[1] && arr[2] && self.v[i - 1][j + 1] != c {
            sum.1 += 1;
        }
        if arr[2] && arr[3] && self.v[i + 1][j + 1] != c {
            sum.1 += 1;
        }
        if arr[3] && arr[0] && self.v[i + 1][j - 1] != c {
            sum.1 += 1;
        }
        sum
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        Self {
            v: value
                .lines()
                .map(|l| l.as_bytes().iter().cloned().collect())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::twelve::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(
            Solution::solve_one(
                "AAAA
BBCD
BBCC
EEEC"
            ),
            140
        );
        assert_eq!(
            Solution::solve_one(
                "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"
            ),
            1930
        );
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(Solution::solve_two("AAAA"), 16);
        assert_eq!(
            Solution::solve_two(
                "AAAA
BBCD
BBCC
EEEC
"
            ),
            80
        );
        assert_eq!(
            Solution::solve_two(
                "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"
            ),
            236
        );
        assert_eq!(
            Solution::solve_two(
                "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
            ),
            368
        );
        assert_eq!(
            Solution::solve_two(
                "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            ),
            1206
        );
    }
}
