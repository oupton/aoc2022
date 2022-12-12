use std::collections::HashSet;

fn get_grid(input: &str) -> Option<Vec<Vec<u32>>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10))
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<Vec<_>>>>()
}

fn __find_visible<'a, I>(trees: I, res: &mut HashSet<u32>)
where
    I: Iterator<Item = (usize, &'a u32)>,
{
    let mut max = i64::MIN;

    for (i, &tree) in trees {
        let height = i64::from(tree);
        if height > max {
            res.insert(i as u32);
            max = height;
        }
    }
}

fn find_visible(trees: &[u32]) -> HashSet<u32> {
    let mut res = HashSet::new();

    __find_visible(trees.iter().enumerate(), &mut res);
    __find_visible(trees.iter().enumerate().rev(), &mut res);

    res
}

pub fn part1(input: &str) -> Option<usize> {
    let mut visible = HashSet::new();
    let grid = get_grid(input)?;
    let nr_cols = grid.get(0)?.len();

    // test visibility from top and bottom
    for col_idx in 0..nr_cols {
        let col: Vec<u32> = grid.iter().map(|r| r[col_idx]).collect();

        for row_idx in find_visible(&col) {
            visible.insert((row_idx, col_idx));
        }
    }

    for (row_idx, row) in grid.iter().enumerate() {
        for col_idx in find_visible(row) {
            visible.insert((row_idx as u32, col_idx as usize));
        }
    }

    Some(visible.len())
}

fn __viewing_distance<'a, I>(height: u32, trees: I) -> usize
where
    I: Iterator<Item = &'a u32>,
{
    let mut distance = 0;

    for &tree in trees {
        distance += 1;
        if tree >= height {
            return distance;
        }
    }

    distance
}

fn scenic_score(r: usize, c: usize, grid: &[Vec<u32>]) -> usize {
    let col = grid.iter().map(|row| row[c]).collect::<Vec<_>>();
    let height = grid[r][c];
    let top = __viewing_distance(height, col[..r].iter().rev());
    let bottom = __viewing_distance(height, col[r + 1..].iter());
    let left = __viewing_distance(height, grid[r][..c].iter().rev());
    let right = __viewing_distance(height, grid[r][c + 1..].iter());

    top * bottom * left * right
}

pub fn part2(input: &str) -> Option<usize> {
    let grid = get_grid(input)?;
    let mut scores = Vec::new();

    for (row_idx, row) in grid.iter().enumerate() {
        for col_idx in 0..row.len() {
            scores.push(scenic_score(row_idx, col_idx, &grid));
        }
    }

    scores.iter().max().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../input/day8/example.txt");
    const TEST: &str = include_str!("../input/day8/test.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), Some(21))
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST), Some(1703))
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), Some(8))
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST), Some(496650))
    }
}
