const INPUT: &str = include_str!("../inputs/day11");

#[derive(Debug, Copy, Clone)]
enum Operation {
    Times(usize),
    Add(usize),
    Squared,
}

impl Operation {
    fn apply_worry_level(self, worry_level: usize) -> usize {
        match self {
            Self::Times(times) => worry_level * times,
            Self::Add(add) => worry_level + add,
            Self::Squared => worry_level * worry_level,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Condition {
    divisible: usize,
    true_throw_to_monkey: usize,
    false_throw_to_monkey: usize,
}

fn parse_monkey_business(
    input: &str,
) -> (Vec<Vec<usize>>, Vec<Operation>, Vec<Condition>, Vec<usize>) {
    let mut monkey_items: Vec<Vec<usize>> = Vec::new();
    let mut monkey_operation: Vec<Operation> = Vec::new();
    let mut monkey_condition: Vec<Condition> = Vec::new();
    for monkey_input in input.split("\n\n") {
        for line in monkey_input.lines() {
            if line.starts_with("Monkey ") {
                monkey_items.push(vec![]);
            } else if line.starts_with("  Starting items: ") {
                let items = line.strip_prefix("  Starting items: ").unwrap().split(", ");
                for item in items {
                    monkey_items
                        .last_mut()
                        .unwrap()
                        .push(item.parse::<usize>().unwrap());
                }
            } else if line.starts_with("  Operation: new = ") {
                let rest = &line[19..];
                let operation = if rest.starts_with("old * old") {
                    Operation::Squared
                } else if rest.starts_with("old + ") {
                    Operation::Add(rest[6..].parse::<usize>().unwrap())
                } else if rest.starts_with("old * ") {
                    Operation::Times(rest[6..].parse::<usize>().unwrap())
                } else {
                    panic!("invalid operation")
                };
                monkey_operation.push(operation);
            } else if line.starts_with("  Test: divisible by ") {
                let divisible = line[21..].parse::<usize>().unwrap();
                monkey_condition.push(Condition {
                    divisible,
                    true_throw_to_monkey: 0,
                    false_throw_to_monkey: 0,
                });
            } else if line.starts_with("    If true: throw to monkey ") {
                let monkey = line[29..].parse::<usize>().unwrap();
                monkey_condition.last_mut().unwrap().true_throw_to_monkey = monkey;
            } else if line.starts_with("    If false: throw to monkey ") {
                let monkey = line[30..].parse::<usize>().unwrap();
                monkey_condition.last_mut().unwrap().false_throw_to_monkey = monkey;
            } else {
                panic!("invalid syntax");
            }
        }
    }
    let monkey_inspections: Vec<usize> = vec![0; monkey_items.len()];
    (
        monkey_items,
        monkey_operation,
        monkey_condition,
        monkey_inspections,
    )
}

fn monkey_business(input: &str, rounds: usize) -> usize {
    let (mut monkey_items, monkey_operation, monkey_condition, mut monkey_inspections) =
        parse_monkey_business(input);
    for _ in 0..rounds {
        for monkey in 0..monkey_items.len() {
            while monkey_items[monkey].len() > 0 {
                monkey_inspections[monkey] += 1;
                let item = monkey_items[monkey].remove(0);
                let worry_item = monkey_operation[monkey].apply_worry_level(item) / 3;
                let condition = monkey_condition[monkey];
                if worry_item % condition.divisible == 0 {
                    monkey_items[condition.true_throw_to_monkey].push(worry_item);
                } else {
                    monkey_items[condition.false_throw_to_monkey].push(worry_item);
                }
            }
        }
    }
    monkey_inspections.sort();
    let active_monkey1 = monkey_inspections.pop().unwrap();
    let active_monkey2 = monkey_inspections.pop().unwrap();
    active_monkey1 * active_monkey2
}

// https://www.includehelp.com/rust/find-the-lcm.aspx
fn lcd(n1: usize, n2: usize) -> usize {
    let mut rem: usize;
    let mut x: usize;
    let mut y: usize;

    if n1 > n2 {
        x = n1;
        y = n2;
    } else {
        x = n2;
        y = n1;
    }

    rem = x % y;

    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }

    n1 * n2 / y
}

fn lcd_monkeys(monkey_condition: Vec<Condition>) -> usize {
    let mut monkey_lcd = monkey_condition.first().unwrap().divisible;
    for condition in monkey_condition {
        monkey_lcd = lcd(monkey_lcd, condition.divisible);
    }
    monkey_lcd
}

fn more_monkey_business(input: &str) -> usize {
    let (mut monkey_items, monkey_operation, monkey_condition, mut monkey_inspections) =
        parse_monkey_business(input);
    let monkey_lcd = lcd_monkeys(monkey_condition.clone());
    for _ in 0..10000 {
        for monkey in 0..monkey_items.len() {
            while monkey_items[monkey].len() > 0 {
                monkey_inspections[monkey] += 1;
                let item = monkey_items[monkey].remove(0);
                let condition = monkey_condition[monkey];
                let worry_item = monkey_operation[monkey].apply_worry_level(item) % monkey_lcd;
                if worry_item % condition.divisible == 0 {
                    monkey_items[condition.true_throw_to_monkey].push(worry_item);
                } else {
                    monkey_items[condition.false_throw_to_monkey].push(worry_item);
                }
            }
        }
    }
    monkey_inspections.sort();
    let active_monkey1 = monkey_inspections.pop().unwrap();
    let active_monkey2 = monkey_inspections.pop().unwrap();
    active_monkey1 * active_monkey2
}

fn main() {
    println!("{}", monkey_business(INPUT, 20));
    println!("{}", more_monkey_business(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn intro1() {
        assert_eq!(monkey_business(TEST_INPUT, 20), 10605);
    }

    #[test]
    fn solution1() {
        assert_eq!(monkey_business(INPUT, 20), 99852);
    }

    #[test]
    fn intro2() {
        assert_eq!(more_monkey_business(TEST_INPUT), 2713310158);
    }

    #[test]
    fn solution2() {
        assert_eq!(more_monkey_business(INPUT), 25935263541);
    }
}
