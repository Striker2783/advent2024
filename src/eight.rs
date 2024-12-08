use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

pub fn run(path: &Path) {
    let s = fs::read_to_string(path).unwrap();
    println!("{}", Solution::solve_one(&s));
    println!("{}", Solution::solve_two(&s));
}
struct Solution {
    map: HashMap<char, Vec<(usize, usize)>>,
    size: (usize, usize),
}
impl Solution {
    pub fn solve_one(s: &str) -> u32 {
        let solution = Solution::from(s);
        let mut set = HashSet::new();
        for v in solution.map.values() {
            for i in 0..(v.len() - 1) {
                let v1 = (v[i].0 as i32, v[i].1 as i32);
                for j in (i + 1)..v.len() {
                    let v2 = (v[j].0 as i32, v[j].1 as i32);
                    let slope = (v1.0 - v2.0, v1.1 - v2.1);
                    let new1 = (v1.0 + slope.0, v1.1 + slope.1);
                    let new2 = (v2.0 - slope.0, v2.1 - slope.1);
                    for pos in [new1, new2] {
                        if pos.0 < 0 || pos.1 < 0 {
                            continue;
                        }
                        if pos.0 as usize > solution.size.0 || pos.1 as usize > solution.size.1 {
                            continue;
                        }
                        set.insert(pos);
                    }
                }
            }
        }
        set.iter().count() as u32
    }
    pub fn solve_two(s: &str) -> u32 {
        let solution = Solution::from(s);
        let mut set = HashSet::new();
        for v in solution.map.values() {
            for i in 0..(v.len() - 1) {
                let v1 = (v[i].0 as i32, v[i].1 as i32);
                for j in (i + 1)..v.len() {
                    let v2 = (v[j].0 as i32, v[j].1 as i32);
                    let slope = (v1.0 - v2.0, v1.1 - v2.1);
                    for mult in -100..100 {
                        let pos = (v1.0 - slope.0 * mult, v1.1 - slope.1 * mult);
                        if pos.0 < 0 || pos.1 < 0 {
                            continue;
                        }
                        if pos.0 as usize > solution.size.0 || pos.1 as usize > solution.size.1 {
                            continue;
                        }
                        set.insert(pos);
                    }
                }
            }
        }
        set.iter().count() as u32
    }
}

impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut map = HashMap::new();
        let mut size = (0, 0);
        for (i, line) in value.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                size = (size.0.max(i), size.1.max(j));
                if c == '.' {
                    continue;
                }
                map.entry(c)
                    .and_modify(|v: &mut Vec<(usize, usize)>| v.push((i, j)))
                    .or_insert(vec![(i, j)]);
            }
        }
        Self { map, size }
    }
}

#[cfg(test)]
mod tests {
    use crate::eight::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(
            Solution::solve_one(
                "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"
            ),
            14
        );
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(
            Solution::solve_two(
                "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"
            ),
            34
        );
    }
}
