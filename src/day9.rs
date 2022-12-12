use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord(isize, isize);

fn next(head: &Coord, tail: &Coord) -> Coord {
    let mut r = *tail;
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    if dx.abs() > 1 || dy.abs() > 1 {
        r.0 += dx.clamp(-1, 1);
        r.1 += dy.clamp(-1, 1);
    }

    r
}

fn nr_visited(input: &str, rope_len: usize) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut rope = vec![Coord(0, 0); rope_len];

    for cmd in input.lines() {
        let (dir, v) = cmd.split_once(' ')?;
        let val = v.parse::<isize>().ok()?;

        for _ in 0..val {
            let head = rope[0];
            rope[0] = match dir {
                "U" => Coord(head.0, head.1 + 1),
                "D" => Coord(head.0, head.1 - 1),
                "L" => Coord(head.0 - 1, head.1),
                "R" => Coord(head.0 + 1, head.1),
                _ => panic!("owwie!"),
            };

            for i in 1..rope_len {
                rope[i] = next(&rope[i - 1], &rope[i]);
            }

            visited.insert(rope[rope_len - 1]);
        }
    }

    Some(visited.len())
}

pub fn part1(input: &str) -> Option<usize> {
    nr_visited(input, 2)
}

pub fn part2(input: &str) -> Option<usize> {
    nr_visited(input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../input/day9/example1.txt");
    const EXAMPLE2: &str = include_str!("../input/day9/example2.txt");
    const TEST: &str = include_str!("../input/day9/test.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE1), Some(13))
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST), Some(6311))
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(EXAMPLE1), Some(1))
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(EXAMPLE2), Some(36))
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST), Some(2482))
    }
}
