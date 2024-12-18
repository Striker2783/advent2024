use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs,
    path::Path,
};

pub fn run(f: &Path) {
    let s = fs::read_to_string(f).unwrap();
    let mut solution = Solution::from(s.as_str());
    solution.size = (70,70);
    solution.simulate = 1024;
    println!("{}", solution.solve_one());
    let two = solution.solve_two();
    println!("{},{}", two.0,two.1);
}
struct Solution {
    size: (usize, usize),
    simulate: u32,
    bytes: Vec<(usize, usize)>,
}
impl Solution {
    fn solve_one(&self) -> u32 {
        let set: HashSet<_> = self
            .bytes
            .iter()
            .take(self.simulate as usize)
            .cloned()
            .collect();
        let mut pq = BinaryHeap::new();
        let mut distances = vec![vec![u32::MAX; self.size.1 + 1]; self.size.0 + 1];
        pq.push(Reverse((0, 0, 0)));
        while let Some(Reverse((d, i, j))) = pq.pop() {
            if set.contains(&(i, j)) || d >= distances[i][j] {
                continue;
            } else if (i,j) == (self.size.0, self.size.1) {
                return d;
            }
            distances[i][j] = d;
            if i > 0 {
                pq.push(Reverse((d + 1, i - 1, j)));
            }
            if j > 0 {
                pq.push(Reverse((d + 1, i, j - 1)));
            }
            if i < self.size.0 {
                pq.push(Reverse((d + 1, i + 1, j)));
            }
            if j < self.size.1 {
                pq.push(Reverse((d + 1, i, j + 1)));
            }
        }
        distances[self.size.0][self.size.1]
    }
    fn solve_two(&mut self) -> (usize,usize) {
        for i in 1..=self.bytes.len() {
            self.simulate = i as u32;
            if self.solve_one() != u32::MAX {
                continue;
            }
            return self.bytes[i-1];
        }
        unreachable!()
    }
}
impl Default for Solution {
    fn default() -> Self {
        Self {
            size: (10, 10),
            simulate: 1024,
            bytes: Default::default(),
        }
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut s = Self::default();
        for line in value.lines() {
            let mut split = line.split(',');
            s.bytes.push((
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            ));
        }
        s
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solve_one() {
        let mut solution = Solution::from(
            "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
",
        );
        solution.size = (6, 6);
        solution.simulate = 12;
        assert_eq!(solution.solve_one(), 22);
        assert_eq!(solution.solve_two(), (6,1));
    }
}
