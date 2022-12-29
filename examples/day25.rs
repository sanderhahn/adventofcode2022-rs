const INPUT: &str = include_str!("../inputs/day25");

fn parse_snafu(input: &str) -> isize {
    let chars: Vec<char> = input.chars().collect();
    let mut num = 0;
    for (index, digit) in chars.iter().enumerate() {
        let pos = chars.len() - index - 1;
        num += 5_isize.pow(pos as u32)
            * match digit {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("invalid snafu digit"),
            };
    }
    num
}

fn to_snafu(num: isize) -> String {
    let mut rest = num;
    let mut digits = "".to_string();
    while rest > 0 {
        let mod_5 = rest % 5;
        digits.insert(
            0,
            match mod_5 {
                4 => {
                    rest += 5;
                    '-'
                }
                3 => {
                    rest += 5;
                    '='
                }
                2 => '2',
                1 => '1',
                0 => '0',
                _ => panic!("internal error"),
            },
        );
        rest /= 5;
    }
    digits
}

fn sum_snafu(input: &str) -> String {
    let sum = input.lines().map(|line| parse_snafu(line)).sum();
    to_snafu(sum)
}

fn main() {
    println!("{}", sum_snafu(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    #[test]
    fn intro1() {
        assert_eq!(sum_snafu(TEST_INPUT), "2=-1=0");
    }

    #[test]
    fn solution1() {
        assert_eq!(sum_snafu(INPUT), "2==221=-002=0-02-000");
    }
}
