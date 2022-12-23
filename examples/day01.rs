const INPUT: &str = include_str!("../inputs/day01");

fn total(carrying: &str) -> usize {
    let mut total: usize = 0;
    for carrage in carrying.lines() {
        let calories: usize = carrage.parse().unwrap();
        total += calories;
    }
    total
}

fn parse_input(input: &str) -> Vec<usize> {
    let mut elfs = Vec::new();
    for carrying in input.split("\n\n") {
        elfs.push(total(carrying));
    }
    elfs.sort_by(|a, b| b.cmp(a));
    elfs
}

pub fn elf_most_carrying(elfs: Vec<usize>) -> usize {
    *elfs.first().unwrap()
}

pub fn top_three_elfs_most_carrying(elfs: Vec<usize>) -> usize {
    elfs[0..=2].iter().sum()
}

fn day01a() -> usize {
    elf_most_carrying(parse_input(INPUT))
}

fn day01b() -> usize {
    top_three_elfs_most_carrying(parse_input(INPUT))
}

fn main() {
    println!("{}", day01a());
    println!("{}", day01b());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn intro1() {
        assert_eq!(elf_most_carrying(parse_input(TEST_INPUT)), 24000);
    }

    #[test]
    fn solution1() {
        assert_eq!(day01a(), 71124);
    }

    #[test]
    fn intro2() {
        assert_eq!(top_three_elfs_most_carrying(parse_input(TEST_INPUT)), 45000);
    }

    #[test]
    fn solution2() {
        assert_eq!(top_three_elfs_most_carrying(parse_input(INPUT)), 204639);
    }
}
