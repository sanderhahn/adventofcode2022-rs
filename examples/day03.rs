const INPUT: &str = include_str!("../inputs/day03");

fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 26 + 1,
        _ => panic!("invalid item"),
    }
}

fn find_shared_item(rucksack: &str) -> u32 {
    let middle = rucksack.len() >> 1;
    let left_compartment = &rucksack[0..middle];
    let right_compartment = &rucksack[middle..];
    for item in left_compartment.chars() {
        match right_compartment.find(item) {
            Some(_) => {
                return priority(item);
            }
            _ => continue,
        }
    }
    panic!("no common item");
}

pub fn sum_priorities(input: &str) -> u32 {
    let mut total = 0;
    for rucksack in input.trim().split("\n") {
        total += find_shared_item(rucksack);
    }
    total
}

pub fn day03a() -> u32 {
    sum_priorities(INPUT)
}

fn find_shared_item_grouped_by_three(rucksacks: Vec<&str>) -> u32 {
    if let [one, two, three] = *rucksacks {
        for item in one.chars() {
            match two.find(item) {
                Some(_) => match three.find(item) {
                    Some(_) => {
                        return priority(item);
                    }
                    _ => continue,
                },
                _ => continue,
            }
        }
    }
    panic!("no common item");
}

pub fn sum_priorities_grouped_by_three(input: &str) -> u32 {
    let mut total = 0;
    let rucksacks: Vec<&str> = input.trim().split("\n").collect();
    for rucksacks in rucksacks.chunks(3) {
        total += find_shared_item_grouped_by_three(rucksacks.to_vec());
    }
    total
}

pub fn day03b() -> u32 {
    sum_priorities_grouped_by_three(INPUT)
}

fn main() {
    println!("{}", day03a());
    println!("{}", day03b());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn intro1() {
        assert_eq!(sum_priorities(TEST_INPUT), 157);
    }

    #[test]
    fn solution1() {
        assert_eq!(day03a(), 7878);
    }

    #[test]
    fn intro2() {
        assert_eq!(sum_priorities_grouped_by_three(TEST_INPUT), 70);
    }

    #[test]
    fn solution2() {
        assert_eq!(day03b(), 2760);
    }
}
