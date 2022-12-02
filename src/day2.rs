enum Play {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for Play {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err("unrecognized play"),
        }
    }
}

fn score(game: &str) -> Option<isize> {
    let mut plays = game.split(' ');
    let theirs = Play::try_from(plays.next()?).ok()?;
    let ours = Play::try_from(plays.next()?).ok()?;

    match (ours, theirs) {
        (Play::Rock, Play::Rock) => Some(4),
        (Play::Rock, Play::Paper) => Some(1),
        (Play::Rock, Play::Scissors) => Some(7),
        (Play::Paper, Play::Rock) => Some(8),
        (Play::Paper, Play::Scissors) => Some(2),
        (Play::Paper, Play::Paper) => Some(5),
        (Play::Scissors, Play::Rock) => Some(3),
        (Play::Scissors, Play::Paper) => Some(9),
        (Play::Scissors, Play::Scissors) => Some(6),
    }
}

pub fn part1(input: &str) -> isize {
    input.lines().map(|x| score(x).unwrap()).sum()
}

const LOSE: &str = "X";
const DRAW: &str = "Y";
const WIN: &str = "Z";

fn get_play(code: &str) -> Option<&str> {
    let mut plays = code.split(' ');
    let theirs = Play::try_from(plays.next()?).ok()?;
    let goal = plays.next().unwrap();

    match (theirs, goal) {
        (Play::Rock, LOSE) => Some("A Z"),
        (Play::Rock, DRAW) => Some("A X"),
        (Play::Rock, WIN) => Some("A Y"),
        (Play::Paper, LOSE) => Some("B X"),
        (Play::Paper, DRAW) => Some("B Y"),
        (Play::Paper, WIN) => Some("B Z"),
        (Play::Scissors, LOSE) => Some("C Y"),
        (Play::Scissors, DRAW) => Some("C Z"),
        (Play::Scissors, WIN) => Some("C X"),
        (_, _) => None,
    }
}

pub fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|x| get_play(x).unwrap())
        .map(|x| score(x).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/day2/example.txt");
    const TEST: &str = include_str!("../input/day2/test.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 15)
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST), 8890)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 12)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST), 10238)
    }
}
