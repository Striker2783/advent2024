use std::{
    fs::{self},
    path::Path,
};

pub fn run(f: &Path) {
    let s = fs::read_to_string(f).unwrap();
    println!("{}", solve_one(&s));
    println!("{}", solve_two(&s));
}
const DIRECTIONS: [[i32; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];
const NAME: [u8; 4] = [b'X', b'M', b'A', b'S'];
pub fn solve_one(s: &str) -> u32 {
    let v: Vec<Vec<_>> = s
        .lines()
        .map(|s| s.as_bytes().into_iter().cloned().collect())
        .collect();

    let mut sum = 0;
    for (i, row) in v.iter().enumerate() {
        for (j, _) in row.iter().cloned().enumerate() {
            for [x, y] in DIRECTIONS {
                for r in 0..NAME.len() {
                    let i = (i as i32) + (x * r as i32);
                    let j = (j as i32) + (y * r as i32);
                    if i < 0 || j < 0 || i >= v.len() as i32 || j >= row.len() as i32 {
                        break;
                    }
                    if v[i as usize][j as usize] == NAME[r] {
                        if r == NAME.len() - 1 {
                            sum += 1;
                        }
                        continue;
                    }
                    break;
                }
            }
        }
    }
    sum
}
pub fn solve_two(s: &str) -> u32 {
    let v: Vec<Vec<_>> = s
        .lines()
        .map(|s| s.as_bytes().into_iter().cloned().collect())
        .collect();
    let mut sum = 0;
    for (i, row) in v.iter().enumerate() {
        if i == 0 || i >= v.len() - 1 {
            continue;
        }
        for (j, c) in row.iter().cloned().enumerate() {
            if c != b'A' || j == 0 || j >= row.len() - 1 {
                continue;
            }
            let direction = [
                [i - 1, j - 1],
                [i - 1, j + 1],
                [i + 1, j - 1],
                [i + 1, j + 1],
            ];
            let m_count = direction.iter().filter(|[x, y]| v[*x][*y] == b'M').count();
            let s_count = direction.iter().filter(|[x, y]| v[*x][*y] == b'S').count();
            if m_count != 2 || s_count != 2 {
                continue;
            }
            if v[i - 1][j - 1] == b'M' && v[i - 1][j + 1] == b'M' {
                sum += 1;
            } else if v[i - 1][j + 1] == b'M' && v[i + 1][j + 1] == b'M' {
                sum += 1;
            } else if v[i + 1][j + 1] == b'M' && v[i + 1][j - 1] == b'M' {
                sum += 1;
            } else if v[i + 1][j - 1] == b'M' && v[i - 1][j - 1] == b'M' {
                sum += 1;
            }
        }
    }
    sum
}
#[cfg(test)]
mod tests {
    use crate::four::{solve_one, solve_two};

    #[test]
    fn test_solve_one() {
        assert_eq!(solve_one("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX\n"), 18);
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(
            solve_two(
                ".M.S......\n..A..MSMS.\n.M.S.MAA..\n..A.ASMSM.\n.M.S.M....\n..........\nS.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M.\n..........\n"
            ),
            9
        );
    }
}
