const INPUT: &str = include_str!("../inputs/day05");

#[derive(Debug, Clone)]
struct Supply {
    stacks: Vec<Vec<char>>,
    cranelift: CrateLift,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CrateLift {
    Singular,
    Multiple,
}

impl Supply {
    fn new(stacks: &str, cranelift: CrateLift) -> Supply {
        let mut lines: Vec<&str> = stacks.split("\n").collect();
        let size = lines.pop().unwrap().len() / 4 + 1;
        lines.reverse();
        let mut stacks = vec![Vec::new(); size];
        for line in lines {
            for i in 0..size {
                let index = i * 4 + 1;
                if line.len() >= index {
                    let letter = line.as_bytes()[index] as char;
                    if letter != ' ' {
                        stacks[i].push(letter);
                    }
                }
            }
        }
        Supply { stacks, cranelift }
    }

    fn mov(&mut self, number: usize, from: usize, to: usize) {
        if self.cranelift == CrateLift::Singular {
            for _ in 0..number {
                let letter = self.stacks[from - 1].pop();
                self.stacks[to - 1].push(letter.unwrap());
            }
        } else {
            let mut letters = vec![];
            for _ in 0..number {
                letters.push(self.stacks[from - 1].pop().unwrap());
            }
            letters.reverse();
            for letter in letters {
                self.stacks[to - 1].push(letter);
            }
        }
    }

    fn execute(&mut self, program: &str) {
        for line in program.lines() {
            let parts: Vec<&str> = line.split(" ").collect();
            let (number, from, to) = (parts[1], parts[3], parts[5]);
            self.mov(
                number.parse::<usize>().unwrap(),
                from.parse::<usize>().unwrap(),
                to.parse::<usize>().unwrap(),
            );
        }
    }

    fn word(self) -> String {
        let mut word = Vec::new();
        for stack in self.stacks {
            word.push(*stack.last().unwrap());
        }
        word.into_iter().collect()
    }
}

pub fn move_crates(input: &str, cranelift: CrateLift) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.len() == 2 {
        let (stacks, program) = (parts[0], parts[1]);
        let mut supply = Supply::new(stacks, cranelift);
        supply.execute(program);
        supply.word()
    } else {
        panic!("invalid input")
    }
}

fn day05a() {
    println!("{}", move_crates(INPUT, CrateLift::Singular));
}

fn day05b() {
    println!("{}", move_crates(INPUT, CrateLift::Multiple));
}

fn main() {
    day05a();
    day05b();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn intro1() {
        assert_eq!(move_crates(TEST_INPUT, CrateLift::Singular), "CMZ");
    }

    #[test]
    fn solution1() {
        assert_eq!(move_crates(INPUT, CrateLift::Singular), "QPJPLMNNR");
    }

    #[test]
    fn intro2() {
        assert_eq!(move_crates(TEST_INPUT, CrateLift::Multiple), "MCD");
    }

    #[test]
    fn solution2() {
        assert_eq!(move_crates(INPUT, CrateLift::Multiple), "BQDNWJPVJ");
    }
}
