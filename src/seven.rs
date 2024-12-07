use std::{fs, path::Path};

pub fn run(path: &Path) {
    let s = fs::read_to_string(path).unwrap();
    println!("{}", Solution::solve_one(&s));
    println!("{}", Solution::solve_two(&s));
}
#[derive(Debug)]
struct Solution {
    v: Vec<Vec<u64>>,
}
impl Solution {
    pub fn solve_one(s: &str) -> u64 {
        let solution = Self::from(s);
        let mut sum = 0;
        for i in 0..solution.v.len() {
            if Self::can_solve(&solution.v[i], 0, &mut vec![false; solution.v[i].len() - 2]) {
                sum += solution.v[i][0];
            }
        }
        sum
    }
    pub fn solve_two(s: &str) -> u64 {
        let solution = Self::from(s);
        let mut sum = 0;
        for i in 0..solution.v.len() {
            if Self::can_solve_2(&solution.v[i], 0, &mut vec![0; solution.v[i].len() - 2]) {
                sum += solution.v[i][0];
            }
        }
        sum
    }
    fn can_solve(arr: &[u64], i: usize, v: &mut [bool]) -> bool {
        if i >= arr.len() - 2 {
            let mut value = arr[1];
            for (i, mult) in v.iter().cloned().enumerate() {
                if mult {
                    value *= arr[i + 2];
                } else {
                    value += arr[i + 2];
                }
            }
            return value == arr[0];
        }
        v[i] = false;
        let temp = Self::can_solve(arr, i + 1, v);
        v[i] = true;
        temp || Self::can_solve(arr, i + 1, v)
    }
    fn can_solve_2(arr: &[u64], i: usize, v: &mut [u8]) -> bool {
        if i >= arr.len() - 2 {
            let mut value = arr[1];
            for (i, op) in v.iter().cloned().enumerate() {
                if op == 0 {
                    value *= arr[i + 2];
                } else if op == 1 {
                    value += arr[i + 2];
                } else {
                    let curr = arr[i + 2];
                    let mut n = 1;
                    while n <= curr {
                        value *= 10;
                        n *= 10;
                    }
                    if curr == 0 {
                        value *= 10;
                    }
                    value += curr;
                }
            }
            return value == arr[0];
        }
        v[i] = 0;
        if Self::can_solve_2(arr, i + 1, v) {
            return true;
        }
        v[i] = 1;
        if Self::can_solve_2(arr, i + 1, v) {
            return true;
        }
        v[i] = 2;
        Self::can_solve_2(arr, i + 1, v)
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut s = Solution { v: vec![] };
        for (i, line) in value.lines().enumerate() {
            s.v.push(vec![]);
            let mut split = line.split(": ");
            s.v[i].push(split.next().unwrap().parse().unwrap());
            for n in split.next().unwrap().split(' ') {
                s.v[i].push(n.parse().unwrap());
            }
        }
        s
    }
}
#[cfg(test)]
mod tests {
    use crate::seven::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(
            Solution::solve_one(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
            ),
            3749
        );
    }    #[test]
    fn test_solve_two() {
        assert_eq!(
            Solution::solve_two(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
            ),
            11387
        );
    }
}
