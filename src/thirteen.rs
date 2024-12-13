use std::{fs, path::Path};

pub fn run(path: &Path) {
    let s = fs::read_to_string(path).unwrap();
    println!("{}", Solution::solve_one(&s));
    println!("{}", Solution::solve_two(&s));
}
#[derive(Debug)]
struct Solution {
    v: Vec<[(u32, u32); 3]>,
}
impl Solution {
    fn solve_one(s: &str) -> u32 {
        let solution = Solution::from(s);
        solution
            .v
            .iter()
            .cloned()
            .filter_map(|[a, b, r]| {
                let a = (a.0 as i32, a.1 as i32);
                let b = (b.0 as i32, b.1 as i32);
                let r = (r.0 as i32, r.1 as i32);
                let d = a.0 * b.1 - a.1 * b.0;
                let x = r.0 * b.1 - r.1 * b.0;
                let y = r.1 * a.0 - r.0 * a.1;
                if x % d != 0 || y % d != 0 {
                    return None;
                } else if (x / d) < 0 || y / d < 0 {
                    return None;
                }
                 else {
                    Some(3 * (x / d) as u32 + (y / d) as u32)
                }
            })
            .sum()
    }
    fn solve_two(s: &str) -> u64 {
        let solution = Solution::from(s);
        solution
            .v
            .iter()
            .cloned()
            .filter_map(|[a, b, r]| {
                let a = (a.0 as i64, a.1 as i64);
                let b = (b.0 as i64, b.1 as i64);
                let r = (r.0 as i64 + 10000000000000, r.1 as i64 + 10000000000000);
                let d = a.0 * b.1 - a.1 * b.0;
                let x = r.0 * b.1 - r.1 * b.0;
                let y = r.1 * a.0 - r.0 * a.1;
                if x % d != 0 || y % d != 0 {
                    return None;
                } else if (x / d) < 0 || y / d < 0 {
                    return None;
                } 
                // else if x / d + y / d > 100 {
                //     return None;
                // }
                 else {
                    Some(3 * (x / d) as u64 + (y / d) as u64)
                }
            })
            .sum()
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut v = vec![];
        for game in value.split("\n\n") {
            let mut arr = [(0, 0); 3];
            for (i, line) in game.lines().enumerate() {
                let mut pos = line.split(": ").nth(1).unwrap().split(", ");
                let x = pos.next().unwrap()[2..].parse::<u32>().unwrap();
                let y = pos.next().unwrap()[2..].parse::<u32>().unwrap();
                arr[i] = (x, y);
            }
            v.push(arr);
        }
        Solution { v }
    }
}
#[cfg(test)]
mod tests {
    use crate::thirteen::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(
            Solution::solve_one(
                "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"
            ),
            480
        );
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(
            Solution::solve_two(
                "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"
            ),
            875318608908
        );
    }
}
