use std::collections::{HashMap, HashSet};

fn execute(input: &str, nr_cycles: isize) -> Option<HashMap<isize, isize>> {
    let mut prog = input.lines();
    let mut res = HashMap::new();
    let mut cycle = 1;
    let mut x = 1;

    while cycle <= nr_cycles {
        let insn = prog.next()?;
        let parts = insn.split_whitespace().collect::<Vec<_>>();
        let (mut cpi, dx) = match parts[..] {
            ["addx", imm] => (2, imm.parse::<isize>().ok()?),
            ["noop"] => (1, 0),
            _ => return None,
        };

        loop {
            res.insert(cycle, x);

            cycle += 1;
            cpi -= 1;

            if cpi == 0 {
                x += dx;
                break;
            }
        }
    }

    Some(res)
}

pub fn part1(input: &str) -> Option<isize> {
    let tracepoints = HashSet::from([20, 60, 100, 140, 180, 220]);
    let trace = execute(input, 220)?;

    Some(
        trace
            .iter()
            .filter(|(cycles, _)| tracepoints.contains(cycles))
            .map(|(&cycles, &x)| cycles * x)
            .sum(),
    )
}

pub fn part2(input: &str) -> Option<String> {
    let trace = execute(input, 240)?;
    let pixels = (0..240)
        .map(|px| {
            let d = trace[&(px + 1)] - (px % 40);

            if d.abs() <= 1 {
                "#"
            } else {
                "."
            }
        })
        .collect::<String>();
    let mut display = pixels
        .chars()
        .collect::<Vec<char>>()
        .chunks(40)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    // for a strict match against a file w/ trailing newline
    display.push('\n');
    Some(display)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/day10/example.txt");
    const PART2_EXAMPLE_EXP: &str = include_str!("../input/day10/expected_part2_example.txt");
    const PART2_TEST_EXP: &str = include_str!("../input/day10/expected_part2_test.txt");
    const TEST: &str = include_str!("../input/day10/test.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), Some(13140))
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST), Some(14240))
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), Some(PART2_EXAMPLE_EXP.to_string()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST), Some(PART2_TEST_EXP.to_string()));
    }
}
