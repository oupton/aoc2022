use regex::Regex;
use std::str::FromStr;

fn parse_stacks(s: &str) -> Option<Vec<String>> {
    let mut iter = s.lines().rev();
    let mut stacks = Vec::new();

    // allocate stacks
    for _ in iter.next()?.split_whitespace() {
        stacks.push(String::new());
    }

    for line in iter {
        for (i, element) in line.chars().skip(1).step_by(4).enumerate() {
            if !element.is_whitespace() {
                stacks[i].push(element);
            }
        }
    }

    Some(stacks)
}

#[derive(Copy, Clone, Debug, Default)]
struct Move {
    pub to: usize,
    pub from: usize,
    pub nr: usize,
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^move (?P<nr>\d+) from (?P<from>\d+) to (?P<to>\d+)$")
            .or(Err("couldn't compile regex"))?;
        let caps = re.captures(s).ok_or("regex match failed")?;

        let to = caps["to"].parse().or(Err("couldn't parse destination"))?;
        let from = caps["from"].parse().or(Err("couldn't parse source"))?;
        let nr = caps["nr"].parse().or(Err("couldn't parse nr"))?;

        Ok(Move { to, from, nr })
    }
}

fn parse_moves(s: &str) -> Option<Vec<Move>> {
    s.lines().map(|l| l.parse::<Move>().ok()).collect()
}

pub fn part1(input: &str) -> Option<String> {
    let (s1, s2) = input.split_once("\n\n")?;
    let mut stacks = parse_stacks(s1)?;
    let moves = parse_moves(s2)?;
    let mut res = String::new();

    for m in moves {
        for _ in 0..m.nr {
            let c = stacks[m.from - 1].pop()?;

            stacks[m.to - 1].push(c);
        }
    }

    for mut s in stacks {
        res.push(s.pop()?);
    }

    Some(res)
}

pub fn part2(input: &str) -> Option<String> {
    let (s1, s2) = input.split_once("\n\n")?;
    let mut stacks = parse_stacks(s1)?;
    let moves = parse_moves(s2)?;
    let mut res = String::new();

    for m in moves {
        let mut tmp = String::new();

        for _ in 0..m.nr {
            tmp.push(stacks[m.from - 1].pop()?);
        }

        tmp.chars().rev().for_each(|c| stacks[m.to - 1].push(c));
    }

    for mut s in stacks {
        res.push(s.pop()?);
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/day5/example.txt");
    const TEST: &str = include_str!("../input/day5/test.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), Some(String::from("CMZ")))
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST), Some(String::from("HNSNMTLHQ")))
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), Some(String::from("MCD")))
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST), Some(String::from("RNLFDJMCT")))
    }
}
