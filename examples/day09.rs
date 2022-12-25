use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/day09");

type Position = [i32; 2];

fn move_head(direction: &str, head: &mut Position) {
    match direction {
        "U" => head[1] += 1,
        "D" => head[1] -= 1,
        "R" => head[0] += 1,
        "L" => head[0] -= 1,
        _ => panic!("invalid instruction"),
    }
}

fn follow_head(head: Position, tail: &mut Position) {
    let horizontal_distance = (head[0] - tail[0]).abs();
    let vertical_distance = (head[1] - tail[1]).abs();
    if horizontal_distance > 1 && vertical_distance > 1 {
        if head[0] > tail[0] {
            tail[0] += 1;
        } else {
            tail[0] -= 1;
        }
        if head[1] > tail[1] {
            tail[1] += 1;
        } else {
            tail[1] -= 1;
        }
    } else if horizontal_distance > 1 {
        if vertical_distance == 1 {
            if head[1] > tail[1] {
                tail[1] += 1;
            } else {
                tail[1] -= 1;
            }
        }
        if head[0] > tail[0] {
            tail[0] += 1;
        } else {
            tail[0] -= 1;
        }
    } else if vertical_distance > 1 {
        if horizontal_distance == 1 {
            if head[0] > tail[0] {
                tail[0] += 1;
            } else {
                tail[0] -= 1;
            }
        }
        if head[1] > tail[1] {
            tail[1] += 1;
        } else {
            tail[1] -= 1;
        }
    }
}

fn count_tail_positions(input: &str) -> usize {
    let mut positions: HashSet<Position> = HashSet::new();
    let mut head: Position = [0, 0];
    let mut tail: Position = [0, 0];
    for movement in input.lines() {
        let parts: Vec<&str> = movement.split(" ").collect();
        if parts.len() == 2 {
            let direction = parts[0];
            let motion = parts[1].parse::<i32>().unwrap();
            for _ in 0..motion {
                move_head(direction, &mut head);
                follow_head(head, &mut tail);
                positions.insert(tail);
            }
        } else {
            panic!("invalid pattern")
        }
    }
    positions.len()
}

#[allow(dead_code)]
fn visualize(area: [i32; 4], knots: &[[i32; 2]; 10]) {
    for y in area[2]..=area[3] {
        'a: for x in area[0]..area[1] {
            for (i, knot) in knots.iter().enumerate() {
                if i == 0 {
                    if knot[0] == x && knot[1] == y {
                        print!("H");
                        continue 'a;
                    }
                }
                if knot[0] == x && knot[1] == y {
                    print!("{}", i);
                    continue 'a;
                }
            }
            if x == 0 && y == 0 {
                print!("s");
                continue 'a;
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn count_longer_tail_positions(input: &str) -> usize {
    let mut positions: HashSet<Position> = HashSet::new();
    let mut knots = [[0, 0]; 10];
    for movement in input.lines() {
        let parts: Vec<&str> = movement.split(" ").collect();
        if parts.len() == 2 {
            let direction = parts[0];
            let motion = parts[1].parse::<i32>().unwrap();
            // println!("{} {}", direction, motion);
            for _ in 0..motion {
                move_head(direction, &mut knots[0]);
                for i in 1..knots.len() {
                    follow_head(knots[i - 1], &mut knots[i]);
                }
                positions.insert(*knots.last().unwrap());
            }
            // visualize([-11, 14, -5, 15], &knots);
        } else {
            panic!("invalid pattern")
        }
    }
    positions.len()
}

fn main() {
    println!("{}", count_tail_positions(INPUT));
    println!("{}", count_longer_tail_positions(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn intro1() {
        assert_eq!(count_tail_positions(TEST_INPUT), 13);
    }

    #[test]
    fn solution1() {
        assert_eq!(count_tail_positions(INPUT), 6503);
    }

    const LARGER_TEST_INPUT: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn intro2() {
        assert_eq!(count_longer_tail_positions(LARGER_TEST_INPUT), 36);
    }

    #[test]
    fn solution2() {
        assert_eq!(count_longer_tail_positions(INPUT), 2724);
    }
}
