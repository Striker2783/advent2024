use std::{collections::HashSet, fs, path::Path};

pub fn run(f: &Path) {
    let s = fs::read_to_string(f).unwrap();
    println!("{}", solve_one(&s));
    println!("{}", solve_two(&s));
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn new_pos(&self, i: usize, j: usize) -> (usize, usize) {
        match self {
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
        }
    }
    fn turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
pub fn solve_one(s: &str) -> u32 {
    let s: Vec<Vec<_>> = s.lines().map(|line| line.chars().collect()).collect();
    let (mut i, mut j) = (0, 0);
    for (i2, v) in s.iter().enumerate() {
        for (j2, &c) in v.iter().enumerate() {
            if c == '^' {
                (i, j) = (i2, j2);
            }
        }
    }
    let set = transverse(i, j, &s);
    set.iter().count() as u32
}

fn transverse(mut i: usize, mut j: usize, s: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut direction = Direction::Up;
    let mut set = HashSet::new();
    while i != 0 && j != 0 && i < s.len() - 1 && j < s[0].len() - 1 {
        set.insert((i, j));
        let (i2, j2) = direction.new_pos(i, j);
        if s[i2][j2] == '#' {
            direction = direction.turn();
        } else {
            (i, j) = (i2, j2);
        }
    }
    set.insert((i, j));
    set
}
pub fn solve_two(s: &str) -> u32 {
    let mut s: Vec<Vec<_>> = s.lines().map(|line| line.chars().collect()).collect();
    let (mut i, mut j): (usize, usize) = (0, 0);
    for (i2, v) in s.iter().enumerate() {
        for (j2, &c) in v.iter().enumerate() {
            if c == '^' {
                (i, j) = (i2, j2);
            }
        }
    }
    let transverse = transverse(i, j, &s);
    let mut count = 0;
    for (k, l) in transverse {
        if s[k][l] != '.' {
            continue;
        }
        s[k][l] = '#';
        if is_loop(&s, (i, j)) {
            count += 1;
        }
        s[k][l] = '.';
    }
    count
}
fn is_loop(s: &Vec<Vec<char>>, (mut i, mut j): (usize, usize)) -> bool {
    let mut set = HashSet::new();
    let mut direction = Direction::Up;
    while i != 0 && j != 0 && i < s.len() - 1 && j < s[0].len() - 1 {
        if set.contains(&(i, j, direction)) {
            return true;
        }
        set.insert((i, j, direction));
        let (i2, j2) = direction.new_pos(i, j);
        if s[i2][j2] == '#' {
            direction = direction.turn();
        } else {
            (i, j) = (i2, j2);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::six::{solve_one, solve_two};

    #[test]
    fn test_solve_one() {
        assert_eq!(
            solve_one(
                "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...\n"
            ),
            41
        );
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(
            solve_two(
                "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...\n"
            ),
            6
        );
    }
}
