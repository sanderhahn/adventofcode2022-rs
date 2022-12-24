const INPUT: &str = include_str!("../inputs/day06");

fn is_marker(input: &[u8], size: usize) -> bool {
    for i in 0..size {
        for j in 0..size {
            if i != j && input[i] == input[j] {
                return false
            }
        }
    }
    return true
}

pub fn find_start_of_packet_marker(input: &str) -> usize {
    let input = input.as_bytes();
    for offset in 0..input.len() - 4 {
        if is_marker(&input[offset..], 4) {
            return offset + 4
        }
    }
    panic!("no marker found")
}

fn day06a() {
    println!("{}", find_start_of_packet_marker(INPUT));
}

pub fn find_start_of_message_marker(input: &str) -> usize {
    let input = input.as_bytes();
    for offset in 0..input.len() - 4 {
        if is_marker(&input[offset..], 14) {
            return offset + 14
        }
    }
    panic!("no marker found")
}

fn day06b() {
    println!("{}", find_start_of_message_marker(INPUT));
}

fn main() {
    day06a();
    day06b();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(find_start_of_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_start_of_packet_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_start_of_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_start_of_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn solution1() {
        assert_eq!(find_start_of_packet_marker(INPUT), 1287);
    }

    #[test]
    fn tests2() {
        assert_eq!(find_start_of_message_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_start_of_message_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_start_of_message_marker("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(find_start_of_message_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(find_start_of_message_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn solution2() {
        assert_eq!(find_start_of_message_marker(INPUT), 3716);
    }
}
