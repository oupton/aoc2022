fn sum(elf: &str) -> isize {
    elf.lines().map(|item| item.parse::<isize>().unwrap()).sum()
}

pub fn part1(input: &str) -> isize {
    let elves = input.split("\n\n");

    elves.map(sum).max().unwrap_or_default()
}

pub fn part2(input: &str) -> isize {
    let elves = input.split("\n\n");

    let mut vals = elves.map(sum).collect::<Vec<_>>();
    vals.sort();

    vals.iter().rev().take(3).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/day1/example.txt");
    const TEST: &str = include_str!("../input/day1/test.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 24000)
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST), 69310)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 45000)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST), 206104)
    }
}
