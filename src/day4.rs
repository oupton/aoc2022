use std::str::FromStr;

struct Range(isize, isize);

impl Range {
    pub fn contains_num(&self, i: isize) -> bool {
        self.0 <= i && i <= self.1
    }

    pub fn contains(&self, other: &Range) -> bool {
        self.contains_num(other.0) && self.contains_num(other.1)
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.contains_num(other.0)
            || self.contains_num(other.1)
            || other.contains_num(self.0)
            || other.contains_num(self.1)
    }
}

impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bounds = s
            .split('-')
            .map(|x| x.parse::<isize>().or(Err("couldn't parse integer")))
            .collect::<Result<Vec<_>, Self::Err>>()?;

        if bounds.len() == 2 {
            Ok(Self(bounds[0], bounds[1]))
        } else {
            Err("expected 2 integers")
        }
    }
}

pub fn part1(input: &str) -> Option<isize> {
    let mut sum = 0;

    for line in input.lines() {
        let (s1, s2) = line.split_once(',')?;
        let first = s1.parse::<Range>().ok()?;
        let second = s2.parse::<Range>().ok()?;

        if first.contains(&second) || second.contains(&first) {
            sum += 1;
        }
    }

    Some(sum)
}

pub fn part2(input: &str) -> Option<isize> {
    let mut sum = 0;

    for line in input.lines() {
        let (s1, s2) = line.split_once(',')?;
        let first = s1.parse::<Range>().ok()?;
        let second = s2.parse::<Range>().ok()?;

        if first.overlaps(&second) {
            sum += 1;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/day4/example.txt");
    const TEST: &str = include_str!("../input/day4/test.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), Some(2))
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST), Some(524))
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), Some(4))
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST), Some(798))
    }
}
