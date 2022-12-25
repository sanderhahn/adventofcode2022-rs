const INPUT: &str = include_str!("../inputs/day10");

fn process_signal_strength(cycle: i32, x: i32, signal_strength: &mut i32) {
    if cycle == 20 {
        *signal_strength += cycle * x;
    } else if (cycle - 20) % 40 == 0 {
        *signal_strength += cycle * x;
    }
}

fn execute_program(input: &str) -> i32 {
    let mut cycle = 0;
    let mut x = 1;
    let mut signal_strength = 0;
    for line in input.lines() {
        if line.eq("noop") {
            cycle += 1;
            process_signal_strength(cycle, x, &mut signal_strength);
        } else if line.starts_with("addx ") {
            let value = line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();
            cycle += 1;
            process_signal_strength(cycle, x, &mut signal_strength);
            cycle += 1;
            process_signal_strength(cycle, x, &mut signal_strength);
            x += value;
        }
    }
    signal_strength
}

fn render_sprite(cycle: i32, x: i32, screen: &mut String) {
    let screen_cycle = cycle % 40;
    if screen_cycle >= x && screen_cycle < x + 3 {
        screen.push_str("#");
    } else {
        screen.push_str(".");
    }
    if cycle % 40 == 0 {
        screen.push_str("\n");
    }
}

// has some error on the last column of the crt

fn render_screen(input: &str) -> String {
    let mut screen = "".to_string();
    let mut cycle = 0;
    let mut x = 1;
    for line in input.lines() {
        if line.eq("noop") {
            cycle += 1;
            render_sprite(cycle, x, &mut screen);
        } else if line.starts_with("addx ") {
            let value = line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();
            cycle += 1;
            render_sprite(cycle, x, &mut screen);
            cycle += 1;
            render_sprite(cycle, x, &mut screen);
            x += value;
        }
    }
    screen
}

fn main() {
    println!("{}", execute_program(INPUT));
    print!("{}", render_screen(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn intro1() {
        assert_eq!(execute_program(TEST_INPUT), 13140);
    }

    #[test]
    fn solution1() {
        assert_eq!(execute_program(INPUT), 11820);
    }

    #[test]
    fn intro2() {
        assert_eq!(render_screen(TEST_INPUT), r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
");
    }
}
