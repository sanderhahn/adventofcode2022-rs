use core::cmp::Ordering;

const INPUT: &str = include_str!("../inputs/day13");

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Num(usize),
    List(Vec<Packet>),
}

fn cmp_vec(left: &Vec<Packet>, right: &Vec<Packet>) -> Ordering {
    for (index, item_left) in left.iter().enumerate() {
        match right.get(index) {
            Some(item_right) => {
                match item_left.cmp(&item_right) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => continue,
                    Ordering::Greater => return Ordering::Greater,
                }
            }
            None => return Ordering::Greater,
        }
    }
    if left.len() == right.len() {
        Ordering::Equal
    } else {
        Ordering::Less
    }
}

impl Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            left @ Packet::Num(num_left) => match other {
                Packet::Num(num_right) => num_left.cmp(num_right),
                Packet::List(vec_right) => cmp_vec(&vec![left.clone()], &vec_right),
            },
            Packet::List(inner_left) => match other {
                right @ Packet::Num(_) => cmp_vec(inner_left, &vec![right.clone()]),
                Packet::List(inner_right) => cmp_vec(inner_left, inner_right),
            },
        }
    }
}

fn parse_number(input: &str) -> (&str, Packet) {
    let num_str: String = input.chars().take_while(|ch| ch.is_digit(10)).collect();
    let num = num_str.parse::<usize>().unwrap();
    (&input[num_str.len()..], Packet::Num(num))
}

fn parse_packet(input: &str) -> (&str, Packet) {
    let mut pos = input;
    let mut inner: Vec<Packet> = vec![];

    let left_bracket = pos.chars().next().unwrap();
    if left_bracket != '[' {
        panic!("expected [");
    }
    pos = &pos['['.len_utf8()..];

    loop {
        let ch = pos.chars().next().unwrap();
        match ch {
            '[' => {
                let (rest, packet) = parse_packet(pos);
                inner.push(packet);
                pos = rest;
            }
            '0'..='9' => {
                let (rest, packet) = parse_number(pos);
                inner.push(packet);
                pos = rest;
            }
            ',' => {
                pos = &pos[','.len_utf8()..];
            }
            ']' => break,
            _ => panic!("parse error"),
        }
    }

    let right_bracket = pos.chars().next().unwrap();
    if right_bracket != ']' {
        panic!("expected ]");
    }
    pos = &pos[']'.len_utf8()..];

    (pos, Packet::List(inner))
}

fn count_right_ordered(input: &str) -> usize {
    let mut count = 0;
    for (index, pairs) in input.trim().split("\n\n").enumerate() {
        let packets: Vec<&str> = pairs.split("\n").collect();
        if packets.len() == 2 {
            if packets_in_right_order(packets[0], packets[1]) {
                count += index + 1;
            }
        } else {
            panic!("invalid input");
        }
    }
    count
}

fn packets_in_right_order(left: &str, right: &str) -> bool {
    let (rest, left) = parse_packet(left);
    if rest.len() != 0 {
        panic!("parse error {}", rest);
    }
    let (rest, right) = parse_packet(right);
    if rest.len() != 0 {
        panic!("parse error {}", rest);
    }
    let order = left.cmp(&right);
    order == Ordering::Less || order == Ordering::Equal
}

fn find_decoder_key(input: &str) -> usize {
    let mut packets: Vec<Packet> = input
        .trim()
        .replace("\n\n", "\n")
        .split("\n")
        .map(|s| parse_packet(s).1)
        .collect();
    let divider_packet1 = parse_packet("[[2]]").1;
    let divider_packet2 = parse_packet("[[6]]").1;
    packets.push(divider_packet1.clone());
    packets.push(divider_packet2.clone());
    packets.sort_by(|a, b| a.cmp(&b));
    let index_packet1 = &packets
        .iter()
        .position(|e| divider_packet1.cmp(e) == Ordering::Equal)
        .unwrap() + 1;
    let index_packet2 = &packets
        .iter()
        .position(|e| divider_packet2.cmp(e) == Ordering::Equal)
        .unwrap() + 1;
    index_packet1 * index_packet2
}

fn main() {
    println!("{}", count_right_ordered(INPUT));
    println!("{}", find_decoder_key(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn intro1() {
        assert_eq!(packets_in_right_order("[1,1,3,1,1]", "[1,1,5,1,1]"), true);
        assert_eq!(packets_in_right_order("[[1],[2,3,4]]", "[[1],4]"), true);
        assert_eq!(packets_in_right_order("[9]", "[[8,7,6]]"), false);
        assert_eq!(packets_in_right_order("[[4,4],4,4]", "[[4,4],4,4,4]"), true);
        assert_eq!(packets_in_right_order("[7,7,7,7]", "[7,7,7]"), false);
        assert_eq!(packets_in_right_order("[]", "[3]"), true);
        assert_eq!(packets_in_right_order("[[[]]]", "[[]]"), false);
        assert_eq!(
            packets_in_right_order("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"),
            false
        );
        assert_eq!(count_right_ordered(TEST_INPUT), 13);
    }

    #[test]
    fn solution1() {
        assert_eq!(count_right_ordered(INPUT), 5659);
    }

    #[test]
    fn intro2() {
        assert_eq!(find_decoder_key(TEST_INPUT), 140);
    }

    #[test]
    fn solution2() {
        assert_eq!(find_decoder_key(INPUT), 22110);
    }
}
