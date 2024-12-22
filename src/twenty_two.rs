use std::{collections::HashMap, fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    println!("{}", Solution::solve_one(&s));
    println!("{}", Solution::solve_two_2(&s));
}

struct Solution {
    v: Vec<u64>,
}
impl Solution {
    /// Pathetic Brute Force Solution
    fn solve_two(s: &str) -> u64 {
        let solution = Self::from(s);
        let mut big = 0;
        let mut test_count = 0;
        Self::all_sequences(0, &mut [0; 4], &mut |arr| {
            big = solution.calculate_sum_bananas(arr).max(big);
            if test_count % 1_000 == 0 {
                println!("{test_count}");
            }
            test_count += 1;
        });
        big
    }
    /// A better Brute Force Solution
    fn solve_two_2(s: &str) -> u64 {
        let solution = Self::from(s);
        let mut map = HashMap::new();
        for v in solution.v {
            for (k, v) in Self::calculate_bananas_2(v) {
                map.entry(k).and_modify(|n| *n += v).or_insert(v);
            }
        }
        map.values().cloned().max().unwrap() as u64
    }

    fn calculate_sum_bananas(&self, arr: &[i32; 4]) -> u64 {
        self.v
            .iter()
            .map(|&n| Self::calculate_bananas(n, arr))
            .sum()
    }

    fn calculate_bananas_2(mut n: u64) -> HashMap<[i32; 4], u32> {
        let mut map = HashMap::new();
        let mut arr = [0; 4];
        for i in 0..2000 {
            let new = Self::next(n);
            let diff = (new % 10) as i32 - (n % 10) as i32;
            if i < 3 {
                arr[i] = diff;
            } else if i == 3 {
                arr[i] = diff;
                map.insert(arr, (new % 10) as u32);
            } else {
                arr = [arr[1], arr[2], arr[3], diff];
                map.entry(arr).or_insert((new % 10) as u32);
            }
            n = new;
        }
        map
    }

    fn calculate_bananas(mut n: u64, arr: &[i32; 4]) -> u64 {
        let mut i = 0;
        for _ in 0..2000 {
            let new = Self::next(n);
            if (new % 10) as i32 - (n % 10) as i32 == arr[i] {
                i += 1;
                if i >= 4 {
                    return new % 10;
                }
            } else {
                i = 0;
            }
            n = new;
        }
        return 0;
    }

    fn all_sequences<T: FnMut(&[i32; 4]) -> ()>(i: usize, arr: &mut [i32; 4], f: &mut T) {
        if i >= 4 {
            f(&arr);
            return;
        }
        for n in -9..=9 {
            arr[i] = n;
            Self::all_sequences(i + 1, arr, f);
        }
    }

    fn solve_one(s: &str) -> u64 {
        let solution = Self::from(s);
        solution
            .v
            .into_iter()
            .map(|mut n| {
                for _ in 0..2000 {
                    n = Self::next(n);
                }
                n
            })
            .sum()
    }
    fn next(n: u64) -> u64 {
        let n = ((n << 6) ^ n) & 16777215;
        let n = ((n >> 5) ^ n) & 16777215;
        let n = ((n << 11) ^ n) & 16777215;
        n
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        Self {
            v: value.lines().map(|s| s.parse().unwrap()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::twenty_two::Solution;

    #[test]
    fn test_next() {
        assert_eq!(Solution::next(123), 15887950);
    }
    #[test]
    fn test_solve_2() {
        assert_eq!(Solution::solve_two_2("1
2
3
2024"), 23);
    }
    #[test]
    fn test_calculate_bananas() {
        let arr = [-2, 1, -1, 3];
        assert_eq!(Solution::calculate_bananas(1, &arr), 7);
        assert_eq!(Solution::calculate_bananas(2, &arr), 7);
        assert_eq!(Solution::calculate_bananas(3, &arr), 0);
        assert_eq!(Solution::calculate_bananas(2024, &arr), 9);

        println!("{:?}", Solution::calculate_bananas_2(1));
    }
}
