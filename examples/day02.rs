const INPUT: &str = include_str!("../inputs/day02");

fn score(you: char) -> i32 {
    match you {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("invalid code"),
    }
}

fn round(opponent: char, you: char) -> i32 {
    (match (opponent, you) {
        ('A', 'Y') => 6,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        ('A', 'X') => 3,
        ('B', 'Y') => 3,
        ('C', 'Z') => 3,
        ('A', _) => 0,
        ('B', _) => 0,
        ('C', _) => 0,
        _ => panic!("invalid code"),
    }) + score(you)
}

pub fn total_score(input: &str) -> i32 {
    let mut total = 0;
    for line in input.trim().split("\n") {
        let parts: Vec<char> = line.chars().collect();
        if let [opponent, _, you] = parts[..] {
            total += round(opponent, you);
        }
    }
    total
}

pub fn day02a() -> i32 {
    total_score(INPUT)
}

// A rock
// B paper
// C scissors

// X lose
// Y draw
// Z win

fn round_shape(opponent: char, you: char) -> i32 {
    match (opponent, you) {
        ('A', 'X') => 0 + 3,
        ('B', 'X') => 0 + 1,
        ('C', 'X') => 0 + 2,
        ('A', 'Y') => 3 + 1,
        ('B', 'Y') => 3 + 2,
        ('C', 'Y') => 3 + 3,
        ('A', 'Z') => 6 + 2,
        ('B', 'Z') => 6 + 3,
        ('C', 'Z') => 6 + 1,
        _ => panic!("invalid code"),
    }
}

pub fn total_score_shape(input: &str) -> i32 {
    let mut total = 0;
    for line in input.trim().split("\n") {
        let parts: Vec<char> = line.chars().collect();
        if let [opponent, _, you] = parts[..] {
            total += round_shape(opponent, you);
        }
    }
    total
}

pub fn day02b() -> i32 {
    total_score_shape(INPUT)
}

fn main() {
    println!("{}", day02a());
    println!("{}", day02b());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"A Y
B X
C Z
";

    #[test]
    fn intro1() {
        assert_eq!(total_score(TEST_INPUT), 15);
    }

    #[test]
    fn solution1() {
        assert_eq!(day02a(), 12156);
    }

    #[test]
    fn intro2() {
        assert_eq!(total_score_shape(TEST_INPUT), 12);
    }

    #[test]
    fn solution2() {
        assert_eq!(day02b(), 10835);
    }
}
