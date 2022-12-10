use std::collections::HashMap;

fn compute_dirs(input: &str) -> Option<HashMap<String, usize>> {
    let mut dirs = HashMap::new();
    let mut path = Vec::new();

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();

        match parts[..] {
            ["$", "cd", dst] => {
                if dst == ".." {
                    path.pop()?;
                } else {
                    path.push(dst);
                }
            }
            // The command itself isn't really useful. Just inspect the output
            // instead.
            ["$", "ls"] => continue,
            [s, _] => {
                if s == "dir" {
                    continue;
                }

                let size = s.parse::<usize>().ok()?;
                for i in 0..path.len() {
                    let dir = path[..=i].join("/");

                    *dirs.entry(dir).or_default() += size;
                }
            }
            _ => return None,
        }
    }

    Some(dirs)
}

pub fn part1(input: &str) -> Option<usize> {
    let dirs = compute_dirs(input)?;

    Some(dirs.into_values().filter(|size| *size <= 100_000).sum())
}

const DISK_SIZE: usize = 70_000_000;
const SPACE_REQUIRED: usize = 30_000_000;

pub fn part2(input: &str) -> Option<usize> {
    let dirs = compute_dirs(input)?;
    let used = *dirs.get("/")?;
    let available = DISK_SIZE.checked_sub(used)?;
    let space_required = SPACE_REQUIRED.checked_sub(available)?;

    println!("{}", space_required);

    dirs.into_values()
        .filter(|size| *size >= space_required)
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/day7/example.txt");
    const TEST: &str = include_str!("../input/day7/test.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), Some(95437))
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST), Some(1477771))
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), Some(24933642));
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST), Some(3579501))
    }
}
