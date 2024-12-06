use std::{
    cmp::Ordering, collections::{HashMap, HashSet}, fs, path::Path
};

use regex::Regex;

pub fn run(f: &Path) {
    let s = fs::read_to_string(f).unwrap();
    println!("{}", Solution::solve_one(&s));
    println!("{}", Solution::solve_two_2(&s));
}

struct Solution {
    map: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
}

impl Solution {
    fn solve_one(s: &str) -> u32 {
        let solution = Solution::from(s);
        let mut sum = 0;
        'outer: for v in &solution.updates {
            let mut wrong = HashSet::new();
            for &n in v {
                if wrong.contains(&n) {
                    continue 'outer;
                }
                if let Some(v) = solution.map.get(&n) {
                    for &n in v {
                        wrong.insert(n);
                    }
                }
            }
            sum += v[v.len() / 2];
        }
        sum
    }
    /// Imagine a O(n!) or something like that runtime for one row
    fn solve_two(s: &str) -> u32 {
        let solution = Solution::from(s);
        let mut sum = 0;
        for v_ori in &solution.updates {
            let mut v= v_ori.clone();
            'outer: loop {
                let mut wrong = HashSet::new();
                for &n in &v {
                    if wrong.contains(&n) {
                        let mut new_v = vec![n];
                        for &n2 in &v {
                            if n == n2 {continue;}
                            new_v.push(n2);
                        }
                        v = new_v;
                        continue 'outer;
                    }
                    if let Some(v) = solution.map.get(&n) {
                        for &n in v {
                            wrong.insert(n);
                        }
                    }
                }
                break;
            }
            if v == *v_ori {
                continue;
            }
            sum += v[v.len()/2];
        }
        sum
    }
    /// A much better star 2 that runs in O(n*(log n)^2)
    fn solve_two_2(s: &str) -> u32 {
        let solution = Solution::from(s);
        let mut sum = 0;
        for v_ori in &solution.updates {
            let mut v = v_ori.clone();
            v.sort_by(|&a,&b| {
                if let Some(v) = solution.map.get(&a) {
                    if v.contains(&b) {
                        return Ordering::Greater;
                    }
                }
                if let Some(v) = solution.map.get(&b) {
                    if v.contains(&a) {
                        return Ordering::Less
                    }
                }
                Ordering::Equal
            });
            if v == *v_ori {
                continue;
            }
            sum += v[v.len() / 2];
        }
        sum
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let regex = Regex::new(r"(\r?\n){2}").unwrap();
        let mut split = regex.split(value);
        let rules = split.next().unwrap();
        let updates = split.next().unwrap();
        let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
        rules.lines().for_each(|s| {
            let mut split = s.split("|");
            let dependency = split.next().unwrap().parse::<u32>().unwrap();
            let dependent = split.next().unwrap().parse::<u32>().unwrap();
            map.entry(dependent)
                .and_modify(|v: &mut Vec<u32>| v.push(dependency))
                .or_insert(vec![dependency]);
        });
        Solution {
            map,
            updates: updates
                .lines()
                .map(|line| {
                    line.split(',')
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::five::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(Solution::solve_one("47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47\n"),143);
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(Solution::solve_two("47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47\n"),123);
        assert_eq!(Solution::solve_two_2("47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47\n"),123);
    }
}
