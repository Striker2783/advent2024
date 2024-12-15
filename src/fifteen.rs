use std::{borrow::BorrowMut, fs, mem::swap, path::Path};

pub fn run(f: &Path) {
    let s = fs::read_to_string(f).unwrap();
    println!("{}", Solution::solve_one(&s));
    println!("{}", Solution::solve_two(&s));
}
struct Solution {
    v: Vec<Vec<u8>>,
    curr: (usize, usize),
    moves: Vec<u8>,
}
impl Solution {
    fn solve_one(s: &str) -> u32 {
        let mut solution = Solution::from(s);
        for &m in &solution.moves {
            if m == b'>' {
                let v: Vec<_> = solution.v[solution.curr.0]
                    .iter_mut()
                    .skip(solution.curr.1)
                    .collect();
                if Solution::rearrange(v) {
                    solution.curr = (solution.curr.0, solution.curr.1 + 1);
                }
            } else if m == b'<' {
                let v: Vec<_> = solution.v[solution.curr.0]
                    .iter_mut()
                    .take(solution.curr.1 + 1)
                    .rev()
                    .collect();
                if Solution::rearrange(v) {
                    solution.curr = (solution.curr.0, solution.curr.1 - 1);
                }
            } else if m == b'^' {
                let v: Vec<_> = solution
                    .v
                    .iter_mut()
                    .take(solution.curr.0 + 1)
                    .map(|v| &mut v[solution.curr.1])
                    .rev()
                    .collect();
                if Solution::rearrange(v) {
                    solution.curr = (solution.curr.0 - 1, solution.curr.1);
                }
            } else if m == b'v' {
                let v: Vec<_> = solution
                    .v
                    .iter_mut()
                    .skip(solution.curr.0)
                    .map(|v| &mut v[solution.curr.1])
                    .collect();
                if Solution::rearrange(v) {
                    solution.curr = (solution.curr.0 + 1, solution.curr.1);
                }
            }
        }
        let mut sum = 0;
        for (i, v) in solution.v.iter().enumerate() {
            for (j, &b) in v.iter().enumerate() {
                if b != b'O' {
                    continue;
                }
                sum += i as u32 * 100 + j as u32;
            }
        }
        sum
    }
    fn solve_two(s: &str) -> u32 {
        let mut solution = Solution::from(s);
        solution.widen();
        'outer: for &m in solution.moves.iter() {
            assert!(solution.v[solution.curr.0][solution.curr.1] == b'@');
            if m == b'<' {
                let i = solution.curr.0;
                let mut end = 0;
                for j in (0..solution.curr.1).rev() {
                    if solution.v[i][j] == b'#' {
                        continue 'outer;
                    } else if solution.v[i][j] == b'.' {
                        end = j;
                        break;
                    }
                }
                for j in end..solution.curr.1 {
                    solution.v[i].swap(j, j + 1);
                }
                solution.curr = (solution.curr.0, solution.curr.1 - 1);
            } else if m == b'>' {
                let i = solution.curr.0;
                let mut end = 0;
                for j in solution.curr.1..solution.v[solution.curr.0].len() {
                    if solution.v[i][j] == b'#' {
                        continue 'outer;
                    } else if solution.v[i][j] == b'.' {
                        end = j;
                        break;
                    }
                }
                for j in ((solution.curr.1+1)..=end).rev() {
                    solution.v[i].swap(j, j - 1);
                }
                solution.curr = (solution.curr.0, solution.curr.1 + 1);
            } else if m == b'^' {
                if Self::can_push_up(&solution.v, solution.curr) {
                    Self::push_up(&mut solution.v, solution.curr);
                    solution.curr = (solution.curr.0 - 1, solution.curr.1);
                }
            } else if m == b'v' {
                if Self::can_push_down(&solution.v, solution.curr) {
                    Self::push_down(&mut solution.v, solution.curr);
                    solution.curr = (solution.curr.0 + 1, solution.curr.1);
                }
            }
        }
        let mut sum = 0;
        for (i, v) in solution.v.iter().enumerate() {
            for (j, &b) in v.iter().enumerate() {
                if b != b'[' {
                    continue;
                }
                sum += i as u32 * 100 + j as u32;
            }
        }
        sum
    }
    fn can_push_up(v: &Vec<Vec<u8>>, (i, j): (usize, usize)) -> bool {
        if v[i - 1][j] == b'[' {
            return Self::can_push_up(v, (i - 1, j)) && Self::can_push_up(v, (i - 1, j + 1));
        } else if v[i - 1][j] == b']' {
            return Self::can_push_up(v, (i - 1, j)) && Self::can_push_up(v, (i - 1, j - 1));
        } else if v[i - 1][j] == b'#' {
            return false;
        } else if v[i - 1][j] == b'.' {
            return true;
        }
        unreachable!()
    }
    fn push_up(v: &mut Vec<Vec<u8>>, (i, j): (usize, usize)) {
        if v[i - 1][j] == b'[' {
            Self::push_up(v, (i - 1, j));
            Self::push_up(v, (i - 1, j + 1))
        } else if v[i - 1][j] == b']' {
            Self::push_up(v, (i - 1, j));
            Self::push_up(v, (i - 1, j - 1))
        }
        let temp = v[i][j];
        v[i][j] = v[i - 1][j];
        v[i - 1][j] = temp;
    }
    fn can_push_down(v: &Vec<Vec<u8>>, (i, j): (usize, usize)) -> bool {
        if v[i + 1][j] == b'[' {
            return Self::can_push_down(v, (i + 1, j)) && Self::can_push_down(v, (i + 1, j + 1));
        } else if v[i + 1][j] == b']' {
            return Self::can_push_down(v, (i + 1, j)) && Self::can_push_down(v, (i + 1, j - 1));
        } else if v[i + 1][j] == b'#' {
            return false;
        } else if v[i + 1][j] == b'.' {
            return true;
        }
        unreachable!()
    }
    fn push_down(v: &mut Vec<Vec<u8>>, (i, j): (usize, usize)) {
        if v[i + 1][j] == b'[' {
            Self::push_down(v, (i + 1, j));
            Self::push_down(v, (i + 1, j + 1))
        } else if v[i + 1][j] == b']' {
            Self::push_down(v, (i + 1, j));
            Self::push_down(v, (i + 1, j - 1))
        }
        let temp = v[i][j];
        v[i][j] = v[i + 1][j];
        v[i + 1][j] = temp;
    }
    fn pretty_print(&self) {
        for l in &self.v {
            for &c in l {
                print!("{}", c as char);
            }
            println!()
        }
    }
    fn rearrange(mut v: Vec<&mut u8>) -> bool {
        assert!(*v[0] == b'@');
        let mut end = 0;
        for i in 1..v.len() {
            if *v[i] == b'.' {
                end = i;
                break;
            } else if *v[i] == b'#' {
                return false;
            }
        }
        for i in (1..=end).rev() {
            let temp = *v[i];
            *v[i] = v[i - 1].clone();
            *v[i - 1] = temp;
        }
        true
    }
    fn widen(&mut self) {
        let mut new_v = vec![];
        for (i, v) in self.v.iter().enumerate() {
            new_v.push(vec![]);
            for (_j, b) in v.iter().cloned().enumerate() {
                if b == b'.' || b == b'#' {
                    new_v[i].push(b);
                    new_v[i].push(b);
                } else if b == b'@' {
                    new_v[i].push(b);
                    new_v[i].push(b'.');
                } else if b == b'O' {
                    new_v[i].push(b'[');
                    new_v[i].push(b']');
                }
            }
        }
        self.curr = (self.curr.0, self.curr.1 * 2);
        self.v = new_v;
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut split = value.split("\n\n");
        let v: Vec<Vec<_>> = split
            .next()
            .unwrap()
            .lines()
            .map(|line| line.as_bytes().iter().cloned().collect())
            .collect();
        let mut curr = (0, 0);
        for (i, v) in v.iter().enumerate() {
            for (j, e) in v.iter().enumerate() {
                if *e == b'@' {
                    curr = (i, j);
                }
            }
        }
        let moves = split
            .next()
            .unwrap()
            .lines()
            .flat_map(|line| line.bytes())
            .collect();
        Self { v, moves, curr }
    }
}
#[cfg(test)]
mod tests {
    use crate::fifteen::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(
            Solution::solve_one(
                "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
            ),
            10092
        );
    }
    #[test]
    fn test_solve_two() {
        Solution::solve_two(
            "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
",
        );
        assert_eq!(
            Solution::solve_two(
                "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            ),
            9021
        );
    }
}
