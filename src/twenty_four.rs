use std::{collections::HashMap, fs, path::Path};

pub fn run(p: &Path) {
    let s = fs::read_to_string(p).unwrap();
    println!("{}", Solution::solve_one(&s));
    // println!("{}", Solution::solve_two_2(&s));
}
struct Solution {
    map: HashMap<String, bool>,
    instructions: Vec<(String, String, u32, String)>,
}
impl Solution {
    fn solve_one(s: &str) -> u64 {
        let mut solution = Solution::from(s);
        solution.run();
        let mut n = 0;
        for i in (0..100).rev() {
            let s = format!("z{:02}", i);
            if let Some(&b) = solution.map.get(&s) {
                n <<= 1;
                n += b as u64;
            }
        }
        n
    }
    fn recurse(&mut self, waiting: &mut HashMap<String, Vec<usize>>, idx: usize) {
        let (a, b, i, o) = &self.instructions[idx];
        let a_b = self.map.get(a);
        let b_b = self.map.get(b);
        if a_b.is_none() || b_b.is_none() {
            if a_b.is_none() {
                waiting
                    .entry(a.clone())
                    .and_modify(|v: &mut Vec<usize>| {
                        if !v.contains(&idx) {
                            v.push(idx)
                        }
                    })
                    .or_insert(vec![idx]);
            }
            if b_b.is_none() {
                waiting
                    .entry(b.clone())
                    .and_modify(|v: &mut Vec<usize>| {
                        if !v.contains(&idx) {
                            v.push(idx)
                        }
                    })
                    .or_insert(vec![idx]);
            }
            return;
        }
        let output = match *i {
            1 => a_b.unwrap().clone() && b_b.unwrap().clone(),
            2 => a_b.unwrap().clone() || b_b.unwrap().clone(),
            3 => a_b.unwrap().clone() ^ b_b.unwrap().clone(),
            _ => unreachable!(),
        };
        self.map.insert(o.clone(), output);
        let v = waiting.remove(o);
        if v.is_none() {
            return;
        }
        for idx in v.unwrap() {
            self.recurse(waiting, idx);
        }
    }
    fn run(&mut self) {
        let mut waiting = HashMap::new();
        let len = self.instructions.len();
        for idx in 0..len {
            self.recurse(&mut waiting, idx);
        }
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut split = value.split("\n\n");
        let mut map = HashMap::new();
        for line in split.next().unwrap().lines() {
            let mut split = line.split(": ");
            let a = split.next().unwrap().to_string();
            let b = match split.next().unwrap() {
                "1" => true,
                "0" => false,
                _ => unreachable!(),
            };
            map.insert(a, b);
        }
        let mut instructions = Vec::new();
        for line in split.next().unwrap().lines() {
            if line.is_empty() {
                continue;
            }
            let mut split = line.split(' ');
            let a = split.next().unwrap().to_string();
            let instruction = match split.next().unwrap() {
                "XOR" => 3,
                "OR" => 2,
                "AND" => 1,
                _ => unreachable!(),
            };
            let b = split.next().unwrap().to_string();
            split.next().unwrap();
            let o = split.next().unwrap().to_string();
            instructions.push((a, b, instruction, o));
        }
        Self { map, instructions }
    }
}
#[cfg(test)]
mod tests {
    use crate::twenty_four::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(
            Solution::solve_one(
                "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"
            ),
            4
        );
        assert_eq!(
            Solution::solve_one(
                "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"
            ),
            2024
        );
    }
}
