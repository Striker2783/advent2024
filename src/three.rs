use std::{fs, path::Path};

use regex::Regex;

pub fn run(path: &Path) {
    let s = fs::read_to_string(path).unwrap();
    println!("{}", solve_one(&s));
    println!("{}", solve_two(&s));
}

fn solve_one(s: &str) -> u32 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    regex
        .captures_iter(s)
        .map(|c| c.extract())
        .map(|(_, [one, two])| {
            let (one, two) = (one.parse::<u32>().unwrap(), two.parse::<u32>().unwrap());
            one * two
        })
        .sum::<u32>()
}
fn solve_two(s: &str) -> u32 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut doer = true;
    regex
        .captures_iter(s)
        .map(|m| {
            if m.get(0).unwrap().as_str() == "do()" {
                doer = true;
                0
            } else if m.get(0).unwrap().as_str() == "don't()" {
                doer = false;
                0
            } else if doer {
                let (one, two) = (m.get(1).unwrap().as_str(), m.get(2).unwrap().as_str());
                let (one, two) = (one.parse::<u32>().unwrap(), two.parse::<u32>().unwrap());
                one * two
            } else {
                0
            }
        })
        .sum::<u32>()
}
#[cfg(test)]
mod tests {
    use crate::three::{solve_one, solve_two};

    #[test]
    fn test_solve_one() {
        assert_eq!(
            solve_one("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(
            solve_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
