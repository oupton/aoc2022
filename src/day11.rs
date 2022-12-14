use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Operand {
    Imm(isize),
    Old,
}

impl FromStr for Operand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Self::Old),
            _ => Ok(Self::Imm(s.parse::<isize>().or(Err("not an integer"))?)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
}

impl FromStr for Operator {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => Err("unrecognized operator"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Monkey {
    pub id: usize,
    inventory: Vec<isize>,
    operator: Operator,
    op0: Operand,
    op1: Operand,
    pub divisor: isize,
    target_true: usize,
    target_false: usize,
    pub nr_inspected: usize,
}

impl Monkey {
    pub fn compute_worry(&self, old: isize) -> isize {
        let op0 = match self.op0 {
            Operand::Old => old,
            Operand::Imm(imm) => imm,
        };
        let op1 = match self.op1 {
            Operand::Old => old,
            Operand::Imm(imm) => imm,
        };

        match self.operator {
            Operator::Add => op0 + op1,
            Operator::Multiply => op0 * op1,
        }
    }

    pub fn run_iteration(&mut self, queues: &mut HashMap<usize, Vec<isize>>, reduce_worry: bool) {
        while let Some(item) = self.inventory.pop() {
            let mut next = self.compute_worry(item);

            if reduce_worry {
                next /= 3;
            }

            if next % self.divisor == 0 {
                queues.entry(self.target_true).or_default().push(next);
            } else {
                queues.entry(self.target_false).or_default().push(next);
            }

            self.nr_inspected += 1;
        }
    }

    pub fn catch(&mut self, item: isize) {
        self.inventory.push(item);
    }
}

const MONKEY_RE: &str = r"^Monkey (?P<id>\d+):\n\s+Starting items:(?P<items>[\d, ]+)\n\s+Operation: new = (?P<op0>\S+) (?P<operator>[+\*]) (?P<op1>\S+)\n\s+Test: divisible by (?P<divisor>\d+)\n\s+If true: throw to monkey (?P<target_true>\d+)\n\s+If false: throw to monkey (?P<target_false>\d+)$";

impl FromStr for Monkey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(MONKEY_RE).or(Err("regex compilation"))?;
        let caps = re.captures(s).ok_or("regex match")?;

        let id = caps
            .name("id")
            .ok_or("id")?
            .as_str()
            .parse::<usize>()
            .or(Err("id"))?;
        let inventory = caps
            .name("items")
            .ok_or("items")?
            .as_str()
            .trim()
            .split(", ")
            .map(|s| s.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()
            .or(Err("inventory"))?;
        let op0 = caps.name("op0").ok_or("op0")?.as_str().parse::<Operand>()?;
        let operator = caps
            .name("operator")
            .ok_or("operator")?
            .as_str()
            .parse::<Operator>()?;
        let op1 = caps.name("op1").ok_or("op1")?.as_str().parse::<Operand>()?;
        let divisor = caps
            .name("divisor")
            .ok_or("divisor")?
            .as_str()
            .parse::<isize>()
            .or(Err("divisor"))?;
        let target_true = caps
            .name("target_true")
            .ok_or("target_true")?
            .as_str()
            .parse::<usize>()
            .or(Err("target_true"))?;
        let target_false = caps
            .name("target_false")
            .ok_or("target_false")?
            .as_str()
            .parse::<usize>()
            .or(Err("target_false"))?;

        Ok(Self {
            id,
            inventory,
            operator,
            op0,
            op1,
            divisor,
            target_true,
            target_false,
            nr_inspected: 0,
        })
    }
}

fn get_monkeys(input: &str) -> Option<HashMap<usize, Monkey>> {
    let monkeys = input
        .split("\n\n")
        .map(|s| s.parse::<Monkey>().ok())
        .collect::<Option<Vec<_>>>()?;

    Some(
        monkeys
            .iter()
            .cloned()
            .map(|m| (m.id, m))
            .collect::<HashMap<_, _>>(),
    )
}

fn run_iterations(
    monkeys: &mut HashMap<usize, Monkey>,
    nr_iterations: usize,
    reduce_worry: bool,
) -> Option<usize> {
    let mut queues = HashMap::new();
    let mut common_multiplier: isize = monkeys.iter().map(|(_, m)| m.divisor).product();

    if reduce_worry {
        common_multiplier *= 3;
    }

    for _ in 0..nr_iterations {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(&i)?;
            monkey.run_iteration(&mut queues, reduce_worry);

            for (id, queue) in queues.iter_mut() {
                let monkey = monkeys.get_mut(id)?;

                while let Some(mut item) = queue.pop() {
                    item %= common_multiplier;
                    monkey.catch(item);
                }
            }
        }
    }

    let mut inspected = monkeys
        .iter()
        .map(|(_, m)| m.nr_inspected)
        .collect::<Vec<_>>();
    inspected.sort();

    Some(inspected.iter().rev().take(2).product())
}

pub fn part1(input: &str) -> Option<usize> {
    let mut monkeys = get_monkeys(input)?;

    run_iterations(&mut monkeys, 20, true)
}

pub fn part2(input: &str) -> Option<usize> {
    let mut monkeys = get_monkeys(input)?;

    run_iterations(&mut monkeys, 10_000, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/day11/example.txt");
    const TEST: &str = include_str!("../input/day11/test.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), Some(10605))
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST.trim()), Some(57348))
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), Some(2713310158))
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST.trim()), Some(14106266886))
    }
}
