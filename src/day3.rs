use std::collections::HashSet;

fn get_duplicate(inventory: &str) -> Option<u8> {
    let mid = inventory.len() / 2;

    if inventory.len() % 2 != 0 {
        return None;
    }

    let left = inventory[..mid]
        .as_bytes()
        .iter()
        .copied()
        .collect::<HashSet<_>>();
    let right = inventory[mid..]
        .as_bytes()
        .iter()
        .copied()
        .collect::<HashSet<_>>();

    let intersection = left.intersection(&right).copied().collect::<Vec<_>>();

    if intersection.len() == 1 {
        Some(intersection[0])
    } else {
        None
    }
}

fn get_priority(item: u8) -> Option<isize> {
    match item {
        b'a'..=b'z' => Some(((item - b'a') + 1).into()),
        b'A'..=b'Z' => Some(((item - b'A') + 27).into()),
        _ => None,
    }
}

pub fn part1(input: &str) -> Option<isize> {
    let mut sum = 0;

    for line in input.lines() {
        let dup = get_duplicate(line)?;

        sum += get_priority(dup)?;
    }

    Some(sum)
}

fn get_badge(elves: &[&str]) -> Option<u8> {
    let mut iter = elves.iter();
    let mut res = iter
        .next()?
        .as_bytes()
        .iter()
        .copied()
        .collect::<HashSet<_>>();

    for elf in iter {
        let inventory = elf.as_bytes().iter().copied().collect::<HashSet<_>>();

        res = res
            .intersection(&inventory)
            .copied()
            .collect::<HashSet<_>>();
    }

    if res.len() == 1 {
        res.iter().next().copied()
    } else {
        None
    }
}

pub fn part2(input: &str) -> Option<isize> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut sum = 0;

    for chunk in lines.chunks(3) {
        let elves = chunk.to_vec();

        if elves.len() != 3 {
            return None;
        }

        let badge = get_badge(&elves)?;
        sum += get_priority(badge)?;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/day3/example.txt");
    const TEST: &str = include_str!("../input/day3/test.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), Some(157))
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST), Some(8298))
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), Some(70))
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST), Some(2708))
    }
}
