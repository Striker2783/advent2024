use std::{
    collections::{HashSet, VecDeque},
    fs,
    path::Path,
};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    println!("{}", Solution::solve_one(&s, 100));
    println!("{}", Solution::solve_two(&s, 100));
}

struct Solution {
    grid: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Solution {
    fn solve_one(s: &str, n: u32) -> u32 {
        let s = Self::from(s);
        let d = s.get_distances();
        let mut count = 0;
        for i in 0..s.grid.len() {
            for j in 0..s.grid[i].len() {
                if s.grid[i][j] == b'#' {
                    continue;
                }
                let start = d[i][j];
                if i > 1 && s.grid[i - 2][j] == b'.' {
                    let end = d[i - 2][j];
                    if end > start && end - start - 2 >= n {
                        count += 1;
                    }
                }
                if j > 1 && s.grid[i][j - 2] == b'.' {
                    let end = d[i][j - 2];
                    if end > start && end - start - 2 >= n {
                        count += 1;
                    }
                }
                if i + 2 < s.grid.len() && s.grid[i + 2][j] == b'.' {
                    let end = d[i + 2][j];
                    if end > start && end - start - 2 >= n {
                        count += 1;
                    }
                }
                if j + 2 < s.grid.len() && s.grid[i][j + 2] == b'.' {
                    let end = d[i][j + 2];
                    if end > start && end - start - 2 >= n {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    fn solve_two(s: &str, n: u32) -> u32 {
        let s = Self::from(s);
        let dists = s.get_distances();
        let mut count = 0;
        for i in 0..s.grid.len() {
            for j in 0..s.grid[i].len() {
                if s.grid[i][j] == b'#' {
                    continue;
                }
                let mut visited = HashSet::new();
                let start = dists[i][j];
                let mut queue = VecDeque::new();
                queue.push_back((i + 1, j, 0));
                queue.push_back((i - 1, j, 0));
                queue.push_back((i, j - 1, 0));
                queue.push_back((i, j + 1, 0));
                while let Some((i, j, d)) = queue.pop_front() {
                    if d >= 20 {
                        continue;
                    } else if visited.contains(&(i, j)) {
                        continue;
                    }
                    visited.insert((i, j));
                    if s.grid[i][j] == b'.' && dists[i][j] > start && dists[i][j] - start >= n + d {
                        count += 1;
                    }
                    if i > 0 {
                        queue.push_back((i - 1, j, d + 1));
                    }
                    if j > 0 {
                        queue.push_back((i, j - 1, d + 1));
                    }
                    if i + 1 < s.grid.len() {
                        queue.push_back((i + 1, j, d + 1));
                    }
                    if j + 1 < s.grid[i].len() {
                        queue.push_back((i, j + 1, d + 1));
                    }
                }
            }
        }
        count
    }
    fn get_distances(&self) -> Vec<Vec<u32>> {
        let mut dists = vec![vec![u32::MAX; self.grid[0].len()]; self.grid.len()];
        let mut queue = VecDeque::new();
        queue.push_back((self.start.0, self.start.1, 0));
        while let Some((i, j, d)) = queue.pop_front() {
            if self.grid[i][j] == b'#' {
                continue;
            } else if dists[i][j] <= d {
                continue;
            }
            dists[i][j] = d;
            queue.push_back((i + 1, j, d + 1));
            queue.push_back((i - 1, j, d + 1));
            queue.push_back((i, j + 1, d + 1));
            queue.push_back((i, j - 1, d + 1));
        }
        dists
    }
}

impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut s = Self {
            grid: vec![],
            start: (0, 0),
            end: (0, 0),
        };
        for (i, line) in value.lines().enumerate() {
            if line.is_empty() {
                continue;
            }
            s.grid.push(vec![]);
            for (j, &b) in line.as_bytes().iter().enumerate() {
                if b == b'S' {
                    s.start = (i, j);
                } else if b == b'E' {
                    s.end = (i, j);
                }
                if b == b'#' {
                    s.grid[i].push(b);
                } else {
                    s.grid[i].push(b'.');
                }
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::twenty::Solution;
    const EXAMPLE: &str = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    #[test]
    fn test_solve_one() {
        assert_eq!(Solution::solve_one(EXAMPLE, 2), 44);
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(Solution::solve_two(EXAMPLE, 76), 3);
        assert_eq!(Solution::solve_two(EXAMPLE, 50), 285);
    }
}
