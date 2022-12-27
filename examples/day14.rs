const INPUT: &str = include_str!("../inputs/day14");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Air,
    Rock,
    Sand,
}
type SandBox = Vec<Vec<Cell>>;
type Position = Vec<usize>;

#[allow(dead_code)]
fn print_sandbox(sandbox: &SandBox) {
    for row in sandbox {
        for (x, cell) in row.iter().enumerate() {
            if x < 493 {
                continue;
            }
            let cell = match cell {
                Cell::Air => '.',
                Cell::Rock => '#',
                Cell::Sand => 'o',
            };
            print!("{}", cell);
        }
        print!("\n");
    }
}

fn draw_rocks(sandbox: &mut SandBox, start: &Position, end: &Position) {
    if start[0] == end[0] {
        let x = start[0];
        let mut coords = vec![start[1], end[1]];
        coords.sort();
        for y in coords[0]..=coords[1] {
            sandbox[y][x] = Cell::Rock;
        }
    } else {
        let y = start[1];
        let mut coords = vec![start[0], end[0]];
        coords.sort();
        for x in coords[0]..=coords[1] {
            sandbox[y][x] = Cell::Rock;
        }
    }
}

fn rocks(sandbox: &mut SandBox, start: &str, end: &str) {
    let start: Vec<usize> = start
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let end: Vec<usize> = end
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    draw_rocks(sandbox, &start, &end);
}

fn is_empty(sandbox: &SandBox, position: &Position) -> bool {
    let cell = sandbox[position[1]][position[0]];
    cell == Cell::Air
}

fn simulate_sand(sandbox: &mut SandBox) -> usize {
    let mut sand_to_rest = 0;
    let mut position: Position = vec![500, 0];
    loop {
        if position[1] + 1 == sandbox.len() {
            return sand_to_rest;
        }
        if is_empty(sandbox, &vec![position[0], position[1] + 1]) {
            position[1] += 1;
        } else if is_empty(sandbox, &vec![position[0] - 1, position[1] + 1]) {
            position[0] -= 1;
            position[1] += 1;
        } else if is_empty(sandbox, &vec![position[0] + 1, position[1] + 1]) {
            position[0] += 1;
            position[1] += 1;
        } else {
            sandbox[position[1]][position[0]] = Cell::Sand;
            // print_sandbox(&sandbox);
            sand_to_rest += 1;
            position = vec![500, 0];
            if ! is_empty(sandbox, &position) {
                return sand_to_rest;
            }
        }
    }
}

fn parse_sandbox(input: &str) -> SandBox {
    let mut width = 0;
    let mut height = 0;
    for line in input.lines() {
        let positions: Vec<&str> = line.split(" -> ").collect();
        for coords in &positions {
            let coords: Vec<usize> = coords
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            width = width.max(coords[0]);
            height = height.max(coords[1]);
        }
    }
    let mut sandbox: SandBox = vec![vec![Cell::Air; width + 2 + 200]; height + 2 + 2];
    for line in input.lines() {
        let positions: Vec<&str> = line.split(" -> ").collect();
        for (index, position) in positions.iter().enumerate() {
            if index + 1 < positions.len() {
                rocks(&mut sandbox, position, positions.get(index + 1).unwrap());
            }
        }
    }
    sandbox
}

fn count_sand(input: &str) -> usize {
    let mut sandbox = parse_sandbox(input);
    simulate_sand(&mut sandbox)
}

fn count_floor_sand(input: &str) -> usize {
    let mut sandbox = parse_sandbox(input);
    let width = &sandbox[0].len()-1;
    let height = &sandbox.len()-2;
    draw_rocks(&mut sandbox, &vec![0, height], &vec![width, height]);
    simulate_sand(&mut sandbox)
}

fn main() {
    println!("{}", count_sand(INPUT));
    println!("{}", count_floor_sand(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn intro1() {
        assert_eq!(count_sand(TEST_INPUT), 24);
    }

    #[test]
    fn solution1() {
        assert_eq!(count_sand(INPUT), 838);
    }

    #[test]
    fn intro2() {
        assert_eq!(count_floor_sand(TEST_INPUT), 93);
    }

    #[test]
    fn solution2() {
        assert_eq!(count_floor_sand(INPUT), 27539);
    }
}
