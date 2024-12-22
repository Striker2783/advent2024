use std::{collections::HashMap, fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    println!("{}", Solution::solve_one(&s));
    // println!("{}", Solution::solve_two(&s));
}
const POS: [(usize, usize); 11] = [
    (3, 1),
    (2, 0),
    (2, 1),
    (2, 2),
    (1, 0),
    (1, 1),
    (1, 2),
    (0, 0),
    (0, 1),
    (0, 2),
    (3, 2),
];
const POS2: [(usize, usize); 5] = [(0, 1), (0, 2), (1, 0), (1, 1), (1, 2)];
struct Solution {
    v: Vec<Vec<usize>>,
}
impl Solution {
    fn solve_one(s: &str) -> u64 {
        let solution = Self::from(s);
        for vec in solution.v {
            let mut v = vec![];
            let mut last = 10;
            for b in vec {
                v.append(&mut Self::recurse(b, 2, &mut last));
            }
            println!("{v:?}");
        }
        todo!()
    }
    fn recurse(b: usize, i: u32, last: &mut usize) -> Vec<usize> {
        if i == 0 {
            let a = Self::shortest_1(*last, b);
            *last = b;
            return a;
        }
        let moves = Self::recurse(b, i - 1, last);
        println!("moves: {moves:?}");
        let mut v = vec![];
        for i in 0..(moves.len() - 1) {
            v.append(&mut Self::shortest_2(1, moves[i]));
            v.append(&mut Self::shortest_2(moves[i], 1));
        }
        v.append(&mut Self::shortest_2(1, moves.last().unwrap().clone()));
        println!("after: {v:?}");
        v
    }
    fn shortest_2(from: usize, to: usize) -> Vec<usize> {
        assert!(from == 1 || to == 1);
        if from == 1 {
            match to {
                0 => vec![2, 1],
                1 => vec![1],
                2 => vec![3, 2, 2, 1],
                3 => vec![3, 2, 1],
                4 => vec![3, 1],
                _ => unreachable!(),
            }
        } else {
            match from {
                0 => vec![4, 1],
                1 => vec![1],
                2 => vec![4, 4, 0, 1],
                3 => vec![0, 4, 1],
                4 => vec![0, 1],
                _ => unreachable!(),
            }
        }
    }
    fn shortest_1(from: usize, to: usize) -> Vec<usize> {
        let mut v = vec![];
        let mut curr = POS[from];
        for _ in 0..(curr.0.saturating_sub(POS[to].0)) {
            curr.0 -= 1;
            v.push(0);
        }
        for _ in 0..(POS[to].1.saturating_sub(curr.1)) {
            curr.1 += 1;
            v.push(4);
        }
        for _ in 0..(POS[to].0.saturating_sub(curr.0)) {
            curr.0 += 1;
            v.push(3);
        }
        for _ in 0..(curr.1.saturating_sub(POS[to].1)) {
            curr.1 -= 1;
            v.push(2);
        }
        v.push(1);
        v
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut v = vec![];
        for line in value.lines() {
            v.push(
                line.bytes()
                    .map(|b| if b == b'A' { b'9' + 1 } else { b })
                    .map(|b| (b - b'0') as usize)
                    .collect(),
            )
        }
        Self { v }
    }
}

#[cfg(test)]
mod tests {
    use crate::twenty_one::Solution;

    #[test]
    fn test_shortest_1() {
        Solution::solve_one("0");
    }
}
