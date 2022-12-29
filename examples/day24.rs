const INPUT: &str = include_str!("../inputs/day24");

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
struct Blizard {
    direction: Direction,
    position: Position,
}
#[derive(Debug, PartialEq, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn up(&self) -> Position {
        Position {
            y: self.y - 1,
            ..*self
        }
    }
    fn down(&self) -> Position {
        Position {
            y: self.y + 1,
            ..*self
        }
    }
    fn left(&self) -> Position {
        Position {
            x: self.x - 1,
            ..*self
        }
    }
    fn right(&self) -> Position {
        Position {
            x: self.x + 1,
            ..*self
        }
    }
}

fn move_blizards(bizards: &mut Vec<Blizard>, width: usize, height: usize) {
    for blizard in bizards.iter_mut() {
        match blizard.direction {
            Direction::Up => blizard.position.y -= 1,
            Direction::Down => blizard.position.y += 1,
            Direction::Left => blizard.position.x -= 1,
            Direction::Right => blizard.position.x += 1,
        }
        if blizard.position.x == 0 {
            blizard.position.x = width - 2;
        } else if blizard.position.x == width - 1 {
            blizard.position.x = 1;
        }
        if blizard.position.y == 0 {
            blizard.position.y = height - 2;
        } else if blizard.position.y == height - 1 {
            blizard.position.y = 1;
        }
    }
}

fn is_free(grid: &Vec<Vec<char>>, position: &Position) -> bool {
    return grid[position.y][position.x] == '.';
}

fn add_expedition(expeditions: &mut Vec<Position>, expedition: Position) {
    if !expeditions.contains(&expedition) {
        expeditions.push(expedition);
    }
}

fn plot_blizards(grid: &mut Vec<Vec<char>>, blizards: &Vec<Blizard>) {
    let width = grid[0].len();
    let height = grid.len();
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            grid[y][x] = '.';
        }
    }
    for blizard in blizards {
        grid[blizard.position.y][blizard.position.x] = 'b';
    }
}

fn parse_blizards(input: &str) -> (Vec<Vec<char>>, Vec<Blizard>) {
    let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let width = grid[0].len();
    let height = grid.len();
    let mut blizards = vec![];
    for y in 0..height {
        for x in 0..width {
            match grid[y][x] {
                '^' => blizards.push(Blizard {
                    direction: Direction::Up,
                    position: Position { x, y },
                }),
                'v' => blizards.push(Blizard {
                    direction: Direction::Down,
                    position: Position { x, y },
                }),
                '>' => blizards.push(Blizard {
                    direction: Direction::Right,
                    position: Position { x, y },
                }),
                '<' => blizards.push(Blizard {
                    direction: Direction::Left,
                    position: Position { x, y },
                }),
                _ => (),
            }
        }
    }
    (grid, blizards)
}

fn simulate_expedition(
    grid: &mut Vec<Vec<char>>,
    blizards: &mut Vec<Blizard>,
    start: Position,
    end: Position,
) -> usize {
    let mut expeditions = vec![start];
    let width = grid[0].len();
    let height = grid.len();
    let mut minute = 0;
    loop {
        move_blizards(blizards, width, height);
        plot_blizards(grid, blizards);
        minute += 1;
        for i in 0..expeditions.len() {
            let expedition = expeditions[i].clone();
            if expedition.y < height - 1 && is_free(&grid, &expedition.down()) {
                add_expedition(&mut expeditions, expedition.down());
            }
            if expedition.x < width - 1 && is_free(&grid, &expedition.right()) {
                add_expedition(&mut expeditions, expedition.right());
            }
            if expedition.y > 0 && is_free(&grid, &expedition.up()) {
                add_expedition(&mut expeditions, expedition.up());
            }
            if expedition.x > 0 && is_free(&grid, &expedition.left()) {
                add_expedition(&mut expeditions, expedition.left());
            }
        }
        let mut i = 0;
        while i < expeditions.len() {
            if !is_free(&grid, &expeditions[i]) {
                expeditions.remove(i);
            } else {
                i += 1;
            }
        }
        for expedition in expeditions.iter() {
            if *expedition == end {
                return minute;
            }
        }
    }
}

fn fastest_path(input: &str) -> usize {
    let (mut grid, mut blizards) = parse_blizards(input);
    let width = grid[0].len();
    let height = grid.len();
    simulate_expedition(
        &mut grid,
        &mut blizards,
        Position { x: 1, y: 0 },
        Position {
            x: width - 2,
            y: height - 1,
        },
    )
}

fn fastest_back_and_forth_path(input: &str) -> usize {
    let (mut grid, mut blizards) = parse_blizards(input);
    let width = grid[0].len();
    let height = grid.len();
    let start = Position { x: 1, y: 0 };
    let end = Position {
        x: width - 2,
        y: height - 1,
    };
    let forward = simulate_expedition(&mut grid, &mut blizards, start, end);
    let backward = simulate_expedition(&mut grid, &mut blizards, end, start);
    let returning = simulate_expedition(&mut grid, &mut blizards, start, end);
    forward + backward + returning
}

fn main() {
    println!("{}", fastest_path(INPUT));
    println!("{}", fastest_back_and_forth_path(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    #[test]
    fn intro1() {
        assert_eq!(fastest_path(TEST_INPUT), 18);
    }

    #[test]
    fn solution1() {
        assert_eq!(fastest_path(INPUT), 253);
    }

    #[test]
    fn intro2() {
        assert_eq!(fastest_back_and_forth_path(TEST_INPUT), 54);
    }

    #[test]
    fn solution2() {
        assert_eq!(fastest_back_and_forth_path(INPUT), 794);
    }
}
