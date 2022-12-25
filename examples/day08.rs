const INPUT: &str = include_str!("../inputs/day08");

fn is_visible(x: usize, y: usize, forrest: &Vec<Vec<u8>>) -> bool {
    let width = forrest[0].len();
    let height = forrest.len();
    let tree_height = forrest[y][x];
    let mut visible_left = true;
    for left in 0..x {
        let check = forrest[y][left];
        if check >= tree_height {
            visible_left = false
        }
    }
    let mut visible_right = true;
    for right in x + 1..width {
        let check = forrest[y][right];
        if check >= tree_height {
            visible_right = false
        }
    }
    let mut visible_top = true;
    for top in 0..y {
        let check = forrest[top][x];
        if check >= tree_height {
            visible_top = false
        }
    }
    let mut visible_bottom = true;
    for bottom in y + 1..height {
        let check = forrest[bottom][x];
        if check >= tree_height {
            visible_bottom = false
        }
    }
    visible_left || visible_right || visible_top || visible_bottom
}

fn parse_forrest(input: &str) -> Vec<Vec<u8>> {
    let mut forrest: Vec<Vec<u8>> = input.lines().map(|s| s.as_bytes().to_vec()).collect();
    let width = forrest[0].len();
    let height = forrest.len();
    for x in 0..width {
        for y in 0..height {
            forrest[y][x] -= '0' as u8;
        }
    }
    forrest
}

fn find_visible_trees(input: &str) -> usize {
    let forrest: Vec<Vec<u8>> = parse_forrest(input);
    let width = forrest[0].len();
    let height = forrest.len();
    let outer = width * 2 + (height - 2) * 2;
    let mut inner = 0;
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            if is_visible(x, y, &forrest) {
                inner += 1;
            }
        }
    }
    outer + inner
}

fn calc_scenic_score(x: usize, y: usize, forrest: &Vec<Vec<u8>>) -> usize {
    let width = forrest[0].len();
    let height = forrest.len();
    let tree_height = forrest[y][x];
    let mut visible_left = 0;
    for left in (0..=x-1).rev() {
        let check = forrest[y][left];
        if check >= tree_height {
            visible_left += 1;
            break;
        }
        visible_left += 1;
    }
    let mut visible_right = 0;
    for right in x + 1..width {
        let check = forrest[y][right];
        if check >= tree_height {
            visible_right += 1;
            break;
        }
        visible_right += 1;
    }
    let mut visible_top = 0;
    for top in (0..=y-1).rev() {
        let check = forrest[top][x];
        if check >= tree_height {
            visible_top += 1;
            break;
        }
        visible_top += 1;
    }
    let mut visible_bottom = 0;
    for bottom in y + 1..height {
        let check = forrest[bottom][x];
        if check >= tree_height {
            visible_bottom += 1;
            break;
        }
        visible_bottom += 1;
    }
    visible_left * visible_right * visible_top * visible_bottom
}

fn find_scenic_score(input: &str) -> usize {
    let forrest: Vec<Vec<u8>> = parse_forrest(input);
    let width = forrest[0].len();
    let height = forrest.len();
    let mut top_score = 0;
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            let score = calc_scenic_score(x, y, &forrest);
            if score > top_score {
                top_score = score;
            }
        }
    }
    top_score
}

fn main() {
    println!("{}", find_visible_trees(INPUT));
    println!("{}", find_scenic_score(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"30373
25512
65332
33549
35390
";

    #[test]
    fn intro1() {
        assert_eq!(find_visible_trees(TEST_INPUT), 21);
    }

    #[test]
    fn solution1() {
        assert_eq!(find_visible_trees(INPUT), 1820);
    }

    #[test]
    fn intro2() {
        let forrest: Vec<Vec<u8>> = parse_forrest(TEST_INPUT);
        assert_eq!(calc_scenic_score(2, 1, &forrest), 4);
        assert_eq!(calc_scenic_score(2, 3, &forrest), 8);

        assert_eq!(find_scenic_score(TEST_INPUT), 8);
    }

    #[test]
    fn solution2() {
        assert_eq!(find_scenic_score(INPUT), 385112);
    }
}
