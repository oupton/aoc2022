use std::collections::HashSet;

fn find_marker(input: &str, marker_len: usize) -> Option<usize> {
    let chars = input.chars().collect::<Vec<char>>();

    for (i, window) in chars.windows(marker_len).enumerate() {
        let set = window.iter().copied().collect::<HashSet<char>>();

        if set.len() == marker_len {
            return Some(i + marker_len);
        }
    }

    None
}

pub fn part1(input: &str) -> Option<usize> {
    find_marker(input, 4)
}

pub fn part2(input: &str) -> Option<usize> {
    find_marker(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    // A bit different this time...
    // Each line encodes the input and expected value, comma-delimited.
    const PART1_EXAMPLE: &str = include_str!("../input/day6/part1_example.txt");
    const PART2_EXAMPLE: &str = include_str!("../input/day6/part2_example.txt");

    const TEST: &str = include_str!("../input/day6/test.txt");

    #[test]
    fn part1_example() {
        for line in PART1_EXAMPLE.lines() {
            let (input, exp) = line.split_once(',').unwrap();
            let expected = exp.parse::<usize>().unwrap();

            assert_eq!(part1(input), Some(expected));
        }
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST), Some(1142))
    }

    #[test]
    fn part2_example() {
        for line in PART2_EXAMPLE.lines() {
            let (input, exp) = line.split_once(',').unwrap();
            let expected = exp.parse::<usize>().unwrap();

            assert_eq!(part2(input), Some(expected));
        }
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST), Some(2803))
    }
}
