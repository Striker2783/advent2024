use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    println!("{}", Solution::solve_one(&s));
    println!("{}", Solution::solve_two_2(&s));
}
struct Solution {
    connected: HashMap<String, Vec<String>>,
}
impl Solution {
    fn solve_one(s: &str) -> u32 {
        let solution = Self::from(s);
        let mut set = HashSet::new();
        for s in solution.connected.keys() {
            if s.as_bytes()[0] != b't' {
                continue;
            }
            let c = solution.connected.get(s);
            if c.is_none() {
                continue;
            }
            for s2 in c.unwrap() {
                let c = solution.connected.get(s2);
                if c.is_none() {
                    continue;
                }
                for s3 in c.unwrap() {
                    if !solution.connected.get(s3).unwrap_or(&vec![]).contains(s) {
                        continue;
                    }
                    let mut arr = [s, s2, s3];
                    arr.sort_unstable();
                    set.insert(arr);
                }
            }
        }
        set.iter().count() as u32
    }
    /// RIP Brute Force
    fn solve_two(s: &str) -> String {
        let solution = Self::from(s);
        let mut v = vec![];
        let mut s = solution.recurse(&mut v);
        s.sort_unstable();
        s.join(",")
    }
    /// Bron–Kerbosch algorithm
    fn solve_two_2(s: &str) -> String {
        let solution = Self::from(s);
        let mut p: HashSet<_> = solution.connected.keys().map(|s| s.as_str()).collect();
        let mut best = Vec::new();
        solution.recurse_2(&mut HashSet::new(), &mut p, &mut HashSet::new(), &mut |v| {
            if v.len() > best.len() {
                best = v.clone();
            }
        });
        best.sort_unstable();
        best.join(",")
    }
    fn recurse_2<'a, T: FnMut(&Vec<&'a str>)>(
        &'a self,
        r: &mut HashSet<&'a str>,
        p: &mut HashSet<&'a str>,
        x: &mut HashSet<&'a str>,
        f: &mut T,
    ) {
        if p.is_empty() && x.is_empty() {
            f(&r.iter().cloned().collect());
            return;
        }
        let mut p_clone = p.clone();
        for &v in p.iter() {
            let neighbors: HashSet<_> = match self.connected.get(v) {
                Some(a) => a.iter().map(|s| s.as_str()).collect(),
                None => continue,
            };
            r.insert(&v);
            let mut new_p: HashSet<_> = p_clone.intersection(&neighbors).cloned().collect();
            let mut new_x = p_clone.intersection(&neighbors).cloned().collect();
            self.recurse_2(r, &mut new_p, &mut new_x, f);
            r.remove(&v);
            x.insert(&v);
            p_clone.remove(&v);
        }
    }
    fn recurse<'a>(&'a self, vec: &mut Vec<&'a str>) -> Vec<&'a str> {
        let mut best = vec.clone();
        if vec.is_empty() {
            for k in self.connected.keys() {
                vec.push(k);
                let curr = self.recurse(vec);
                if curr.len() > best.len() {
                    best = curr;
                }
                vec.pop();
            }
            return best;
        }
        if self.connected.get(vec[0]).is_none() {
            return best;
        }
        let mut set = self
            .connected
            .get(vec[0])
            .unwrap()
            .into_iter()
            .collect::<HashSet<_>>();
        for &s in vec.iter().skip(1) {
            if self.connected.get(s).is_none() {
                return best;
            }
            let idk = self.connected.get(s).unwrap();
            set = set
                .intersection(&idk.into_iter().collect())
                .cloned()
                .collect();
        }
        for s in set.iter() {
            if vec.contains(&s.as_str()) {
                continue;
            }
            vec.push(&s);
            let curr = self.recurse(vec);
            if curr.len() > best.len() {
                best = curr;
            }
            vec.pop();
        }
        best
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut connected = HashMap::new();
        for line in value.lines() {
            if line.is_empty() {
                continue;
            }
            let mut split = line.split('-');
            let a = split.next().unwrap();
            let b = split.next().unwrap();
            connected
                .entry(a.to_string())
                .and_modify(|v: &mut Vec<String>| v.push(b.to_string()))
                .or_insert(vec![b.to_string()]);
            connected
                .entry(b.to_string())
                .and_modify(|v: &mut Vec<String>| v.push(a.to_string()))
                .or_insert(vec![a.to_string()]);
        }
        Self { connected }
    }
}
#[cfg(test)]
mod tests {
    use crate::twenty_three::Solution;

    #[test]
    fn test_solve_one() {
        let s = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";
        assert_eq!(Solution::solve_one(s), 7);
        assert_eq!(Solution::solve_two_2(s), "co,de,ka,ta".to_string());
    }
}
