use std::{collections::HashSet, fs, path::Path};

use regex::Regex;

pub fn run(path: &Path) {
    let s = fs::read_to_string(path).unwrap();
    println!("{}", Solution::solve_one(&s, (101, 103)));
    println!("{}", Solution::solve_two(&s, (101, 103)));
}
struct Solution {
    v: Vec<[[i32; 2]; 2]>,
}
impl Solution {
    fn solve_one(s: &str, size: (i32, i32)) -> u32 {
        let solution = Solution::from(s);
        let mut quads = [0, 0, 0, 0];
        for [p, v] in &solution.v {
            let p = [
                (p[0] + v[0] * 100).rem_euclid(size.0),
                (p[1] + v[1] * 100).rem_euclid(size.1),
            ];
            if p[0] > size.0 / 2 && p[1] > size.1 / 2 {
                quads[0] += 1;
            }
            if p[0] < size.0 / 2 && p[1] > size.1 / 2 {
                quads[1] += 1;
            }
            if p[0] > size.0 / 2 && p[1] < size.1 / 2 {
                quads[2] += 1;
            }
            if p[0] < size.0 / 2 && p[1] < size.1 / 2 {
                quads[3] += 1;
            }
        }
        quads.into_iter().product()
    }
    fn solve_two(s: &str, size: (i32, i32)) -> u32 {
        let solution = Solution::from(s);
        for i in 0.. {
            let mut set = solution
                .v
                .iter()
                .map(|[p, v]| {
                    [
                        (p[0] + v[0] * i).rem_euclid(size.0),
                        (p[1] + v[1] * i).rem_euclid(size.1),
                    ]
                })
                .collect::<HashSet<_>>();
            while let Some(curr) = set.iter().next().cloned() {
                let recurse = Self::recurse(&mut set, curr);
                if recurse > 100 {
                    return i as u32;
                }
            }
        }
        unreachable!()
    }
    fn recurse(set: &mut HashSet<[i32; 2]>, curr: [i32; 2]) -> u32 {
        if !set.contains(&curr) {
            return 0;
        }
        set.remove(&curr);
        Self::recurse(set, [curr[0] - 1, curr[1]])
            + Self::recurse(set, [curr[0] + 1, curr[1]])
            + Self::recurse(set, [curr[0], curr[1] - 1])
            + Self::recurse(set, [curr[0], curr[1] + 1])
            + 1
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut v = Vec::new();
        let regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        for line in value.lines() {
            let [p1, p2, v1, v2] = regex.captures(line).unwrap().extract().1;
            v.push([
                [p1.parse().unwrap(), p2.parse().unwrap()],
                [v1.parse().unwrap(), v2.parse().unwrap()],
            ]);
        }
        Self { v }
    }
}
#[cfg(test)]
mod tests {
    use crate::fourteen::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(
            Solution::solve_one(
                "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
",
                (11, 7)
            ),
            12
        );
    }
}
