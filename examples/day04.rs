const INPUT: &str = include_str!("../inputs/day04");

#[derive(Debug, Copy, Clone)]
struct Area {
    start: u32,
    end: u32,
}

impl Area {
    fn new(range: &str) -> Area {
        let parts: Vec<&str> = range.split("-").collect();
        Area {
            start: parts[0].parse::<u32>().unwrap(),
            end: parts[1].parse::<u32>().unwrap(),
        }
    }

    fn complete_overlapping(self, other: &Area) -> bool {
        (self.start <= other.start && self.end >= other.end) ||
        (other.start <= self.start && other.end >= self.end)
    }

    fn no_overlapping(self, other: &Area) -> bool {
        (other.start > self.end) ||
        {self.start > other.end}
    }
}

fn elf_pair_overlapping(elf_pair: &str) -> bool {
    let elf_pairs: Vec<Area> = elf_pair.split(",").map(|s| Area::new(s)).collect();
    elf_pairs[0].complete_overlapping(&elf_pairs[1])
}

pub fn count_overlapping(input: &str) -> u32 {
    let mut overlapping = 0;
    for elf_pair in input.trim().split("\n") {
        if elf_pair_overlapping(elf_pair) {
            overlapping += 1;
        }
    }
    overlapping
}

pub fn day04a() -> u32 {
    count_overlapping(INPUT)
}

fn elf_pair_no_overlapping(elf_pair: &str) -> bool {
    let elf_pairs: Vec<Area> = elf_pair.split(",").map(|s| Area::new(s)).collect();
    elf_pairs[0].no_overlapping(&elf_pairs[1])
}

pub fn count_overlapping_pairs(input: &str) -> u32 {
    let mut overlapping = 0;
    for elf_pair in input.trim().split("\n") {
        if elf_pair_no_overlapping(elf_pair) {
            continue;
        }
        overlapping += 1;
    }
    overlapping
}

pub fn day04b() -> u32 {
    count_overlapping_pairs(INPUT)
}

fn main() {
    println!("{}", day04a());
    println!("{}", day04b());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn intro1() {
        assert_eq!(count_overlapping(TEST_INPUT), 2);
    }

    #[test]
    fn solution1() {
        assert_eq!(day04a(), 448);
    }

    #[test]
    fn intro2() {
        assert_eq!(count_overlapping_pairs(TEST_INPUT), 4);
    }

    #[test]
    fn solution2() {
        assert_eq!(day04b(), 794);
    }
}
