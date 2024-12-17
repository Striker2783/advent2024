use std::{fs, path::Path};

pub fn run(f: &Path) {
    let s = fs::read_to_string(f).unwrap();
    println!("{}", Solution::solve_one(&s));
    println!("{}", Solution::solve_two_2(&s));
}
#[derive(Clone)]
struct Solution {
    registers: [u64; 3],
    instructions: Vec<u8>,
    ip: usize,
}
impl Solution {
    fn solve_one(s: &str) -> String {
        let mut solution = Self::from(s);
        solution
            .helper()
            .into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
    // Doesn't work because branches exist
    fn solve_two(s: &str) -> u64 {
        let original = Solution::from(s);
        let mut curr = original.instructions.len() - 1;
        let mut a = 0;
        loop {
            a += 1 << (curr * 3);
            let mut clone = original.clone();
            clone.registers[0] = a;
            let temp = clone.helper();
            if temp.len() != original.instructions.len() {
                println!("{a} : {:?}, {:?}", temp, original.instructions);
                println!("{curr}");
                return 0;
            }
            if temp == original.instructions {
                return a;
            } else if temp[curr] == original.instructions[curr] {
                curr -= 1;
            }
        }
    }
    fn solve_two_2(s: &str) -> u64 {
        let original = Solution::from(s);
        original.recurse_two(0, 0)
    }
    fn recurse_two(&self, a: u64, curr: usize) -> u64 {
        let mut min = u64::MAX;
        for i in 0..8 {
            if a == 0 && i == 0 {
                continue;
            }
            let mut clone = self.clone();
            clone.registers[0] = a + i;
            let temp = clone.helper();
            if temp[0] == self.instructions[self.instructions.len() - 1 - curr] {
                if curr >= self.instructions.len() - 1 {
                    min = (a + i).min(min);
                } else {
                    min = self.recurse_two((a + i) << 3, curr + 1).min(min);
                }
            }
        }
        min
    }
    fn helper(&mut self) -> Vec<u8> {
        let mut output = Vec::new();
        while self.ip < self.instructions.len() - 1 {
            let operand = self.instructions[self.ip + 1];
            match self.instructions[self.ip] {
                0 => self.registers[0] >>= self.get_value(operand),
                1 => self.registers[1] ^= operand as u64,
                2 => self.registers[1] = self.get_value(operand) % 8,
                3 => {
                    if self.registers[0] != 0 {
                        self.ip = operand as usize;
                        continue;
                    }
                }
                4 => self.registers[1] ^= self.registers[2],
                5 => output.push((self.get_value(operand) % 8) as u8),
                6 => self.registers[1] = self.registers[0] >> self.get_value(operand),
                7 => self.registers[2] = self.registers[0] >> self.get_value(operand),
                _ => unreachable!(),
            };
            self.ip += 2;
        }
        output
    }
    fn get_value(&self, i: u8) -> u64 {
        if i < 4 {
            return i as u64;
        } else {
            return self.registers[i as usize - 4];
        }
    }
}
impl From<&str> for Solution {
    fn from(value: &str) -> Self {
        let mut s = Self {
            registers: [0; 3],
            instructions: Vec::new(),
            ip: 0,
        };
        let mut lines = value.lines();
        for i in 0..3 {
            let line = lines.next().unwrap();
            s.registers[i] = line.split(": ").nth(1).unwrap().parse().unwrap();
        }
        lines.next().unwrap();
        s.instructions = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        s
    }
}
#[cfg(test)]
mod tests {
    use crate::seventeen::Solution;

    #[test]
    fn test_solve_one() {
        assert_eq!(
            Solution::solve_one(
                "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"
            ),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }
    #[test]
    fn test_solve_two() {
        assert_eq!(
            Solution::solve_two_2(
                "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"
            ),
            117440
        );
    }
}
