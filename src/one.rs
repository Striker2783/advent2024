use std::{collections::HashMap, fs, path::Path};

pub fn run(f: &Path) {
    let s = fs::read_to_string(f).unwrap();
    println!("{}", Solution::solve_one(&s));
    println!("{}", Solution::solve_two(&s));
}

struct Solution {}
impl Solution {
    fn solve_one(s: &str) -> u32 {
        let (mut v1, mut v2) = Solution::get_vecs(s);
        v1.sort_unstable();
        v2.sort_unstable();
        v1.iter()
            .cloned()
            .zip(v2.iter().cloned())
            .map(|(i, j)| i.abs_diff(j))
            .sum()
    }
    fn solve_two(s: &str) -> u32 {
        let (v1, v2) = Solution::get_vecs(s);
        let mut map = HashMap::new();
        for v in v2.iter().cloned() {
            map.entry(v).and_modify(|v| *v += 1).or_insert(1);
        }
        v1.iter()
            .cloned()
            .map(|v| v * map.get(&v).unwrap_or(&0u32))
            .sum()
    }
    fn get_vecs(s: &str) -> (Vec<u32>, Vec<u32>) {
        let (mut v1, mut v2) = (vec![], vec![]);
        for line in s.lines() {
            let mut split = line.split("   ");
            v1.push(split.next().unwrap().parse::<u32>().unwrap());
            v2.push(split.next().unwrap().parse::<u32>().unwrap());
        }
        (v1, v2)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(
            Solution::solve_one("3   4\n4   3\n2   5\n1   3\n3   9\n3   3"),
            11
        );
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(
            Solution::solve_two("3   4\n4   3\n2   5\n1   3\n3   9\n3   3"),
            31
        );
    }
}
