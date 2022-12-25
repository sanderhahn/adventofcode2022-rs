use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day07");

fn analyse_session(input: &str) -> HashMap<String, usize> {
    let mut path: Vec<&str> = vec![];
    let mut fs: HashMap<String, usize> = HashMap::new();
    for line in input.lines() {
        if line.starts_with("$ cd ") {
            if line.ends_with("..") {
                path.pop();
            } else {
                path.push(line.strip_prefix("$ cd ").unwrap())
            }
            let fullpath = path[1..].join("/");
            if !fs.contains_key(&fullpath) {
                fs.insert(fullpath, 0);
            }
        } else {
            if line.starts_with("$ ls") || line.starts_with("dir ") {
                // ignore
            } else {
                let parts: Vec<&str> = line.split(" ").collect();
                if parts.len() == 2 {
                    let filesize = parts[0].parse::<usize>().unwrap();
                    for i in 1..=path.len() {
                        let fullpath = path[1..i].join("/");
                        let fullsize = fs.get(&fullpath).unwrap();
                        fs.insert(fullpath, *fullsize + filesize);
                    }
                } else {
                    panic!("invalid pattern")
                }
            }
        }
    }
    fs
}

fn find_small_directories(input: &str) -> usize {
    let fs = analyse_session(input);
    let mut total = 0;
    for value in fs.values() {
        if *value < 100000 {
            total += value;
        }
    }
    total
}

fn find_smallest_directory(input: &str) -> usize {
    let fs = analyse_session(input);
    let unused_space = 70000000 - fs.get("").unwrap();
    let space_to_free = 30000000 - unused_space;
    let mut smallest = usize::MAX;
    for value in fs.values() {
        if *value > space_to_free && *value < smallest {
            smallest = *value;
        }
    }
    smallest
}

fn main() {
    println!("{}", find_small_directories(INPUT));
    println!("{}", find_smallest_directory(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn intro1() {
        assert_eq!(find_small_directories(TEST_INPUT), 95437);
    }

    #[test]
    fn solution1() {
        assert_eq!(find_small_directories(INPUT), 1582412);
    }

    #[test]
    fn intro2() {
        assert_eq!(find_smallest_directory(TEST_INPUT), 24933642);
    }

    #[test]
    fn solution2() {
        assert_eq!(find_smallest_directory(INPUT), 3696336);
    }
}
