use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs,
    path::Path,
    u32,
};

pub fn run(path: &Path) {
    let s = fs::read_to_string(path).unwrap();
    println!("{}", Solution::solve_one(&s));
    println!("{}", Solution::solve_two(&s));
}
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node(u32, (usize, usize), usize);
impl Node {
    fn neighbors(&self) -> [Node; 4] {
        let mut arr = [
            Node(self.0 + 1001, (self.1 .0 - 1, self.1 .1), 0),
            Node(self.0 + 1001, (self.1 .0 + 1, self.1 .1), 1),
            Node(self.0 + 1001, (self.1 .0, self.1 .1 - 1), 2),
            Node(self.0 + 1001, (self.1 .0, self.1 .1 + 1), 3),
        ];
        if self.2 < 4 {
            arr[self.2 as usize].0 -= 1000;
        }
        arr
    }
}
struct Solution {
    maze: Vec<Vec<u8>>,
    end: (usize, usize),
    start: (usize, usize),
}
impl Solution {
    fn solve_one(s: &str) -> u32 {
        let solution = Solution::from(s);
        let mut pq = BinaryHeap::new();
        let mut distances = vec![vec![[u32::MAX; 4]; solution.maze[0].len()]; solution.maze.len()];
        distances[solution.start.0][solution.start.1] = [1000, 1000, 1000, 0];
        for n in Node(0, solution.start, 3).neighbors() {
            if solution.maze[n.1 .0][n.1 .1] == b'#' {
                continue;
            }
            distances[n.1 .0][n.1 .1][n.2] = n.0;
            pq.push(Reverse(n));
        }

        while let Some(e) = pq.pop() {
            for n in e.0.neighbors() {
                if solution.maze[n.1 .0][n.1 .1] == b'#' {
                    continue;
                } else if distances[n.1 .0][n.1 .1][n.2] < n.0 {
                    continue;
                }
                distances[n.1 .0][n.1 .1][n.2] = n.0;
                pq.push(Reverse(n));
            }
        }
        distances[solution.end.0][solution.end.1]
            .iter()
            .min()
            .unwrap()
            .clone()
    }
    fn solve_two(s: &str) -> u32 {
        let solution = Solution::from(s);
        let mut pq = BinaryHeap::new();
        let mut distances = vec![vec![[u32::MAX; 4]; solution.maze[0].len()]; solution.maze.len()];
        distances[solution.start.0][solution.start.1] = [1000, 1000, 1000, 0];
        for n in Node(0, solution.start, 3).neighbors() {
            if solution.maze[n.1 .0][n.1 .1] == b'#' {
                continue;
            }
            distances[n.1 .0][n.1 .1][n.2] = n.0;
            pq.push(Reverse(n));
        }

        while let Some(e) = pq.pop() {
            for n in e.0.neighbors() {
                if solution.maze[n.1 .0][n.1 .1] == b'#' {
                    continue;
                } else if distances[n.1 .0][n.1 .1][n.2] < n.0 {
                    continue;
                }
                distances[n.1 .0][n.1 .1][n.2] = n.0;
                pq.push(Reverse(n));
            }
        }
        let mut stack = vec![];
        let min = *distances[solution.end.0][solution.end.1]
            .iter()
            .min()
            .unwrap();
        let mut set = HashSet::new();
        set.insert((solution.end.0, solution.end.1));
        for (i, &d) in distances[solution.end.0][solution.end.1].iter().enumerate() {
            if d == min {
                stack.push((solution.end.0, solution.end.1, i));
            }
        }
        while let Some((i, j, d)) = stack.pop() {
            for (i2, j2) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                for (i3, &dist) in distances[i2][j2].iter().enumerate() {
                    if dist == u32::MAX {
                        continue;
                    }
                    for n in Node(dist, (i2, j2), i3).neighbors() {
                        if n.1 != (i, j) {
                            continue;
                        } else if n.0 != distances[i][j][d] {
                            continue;
                        }
                        set.insert((i2, j2));
                        stack.push((i2, j2, i3));
                    }
                }
            }
        }
        set.iter().count() as u32
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut maze = vec![];
        let mut end = (0, 0);
        let mut start = (0, 0);
        for (i, line) in value.lines().enumerate() {
            maze.push(vec![]);
            for (j, b) in line.bytes().enumerate() {
                maze[i].push(b);
                if b == b'E' {
                    end = (i, j);
                } else if b == b'S' {
                    start = (i, j);
                }
            }
        }
        Self { maze, end, start }
    }
}
#[cfg(test)]
mod tests {
    use crate::sixteen::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(
            Solution::solve_one(
                "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"
            ),
            7036
        );
        assert_eq!(
            Solution::solve_one(
                "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
            ),
            11048
        );
        assert_eq!(
            Solution::solve_one(
                " ##############
#...########E#
#.#.##.......#
#.#.##.#####.#
#.#..........#
#.####.#######
#.####.......#
#.##########.#
#S...........#
############## "
            ),
            5024
        );
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(
            Solution::solve_two(
                "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            ),
            45
        );
        assert_eq!(
            Solution::solve_two(
                "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
            ),
            64
        );
    }
}
