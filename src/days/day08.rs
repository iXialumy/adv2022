use itertools::izip;
use take_until::TakeUntilExt;

pub fn day08() {
    let input = include_str!("../../input/08.txt");
    let grid = parse_grid(input);

    day08a(&grid);
    day08b(&grid);
}

fn day08a(grid: &[Vec<u32>]) {
    let left_right = left_right(grid);
    let right_left = right_left(grid);
    let top_down = top_down(grid);
    let bottom_up = bottom_up(grid);

    let all = or(left_right, right_left, top_down, bottom_up);
    let count: usize = all
        .iter()
        .map(|row|
            row
                .iter()
                .filter(|x| **x)
                .count())
        .sum();

    println!("Day08a: {count}");
}

fn day08b(grid: &[Vec<u32>]) {
    let max = grid
        .iter()
        .enumerate()
        .flat_map(|(i, v)|
            v
                .iter()
                .enumerate()
                .map(move |(j, _)| (i, j))
        )
        .map(|(i, j)| scenic_score(i, j, grid))
        .max()
        .unwrap();

    println!("Day08b: {max}");
}

fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(to_digit_arr)
        .collect()
}

fn to_digit_arr(line: &str) -> Vec<u32> {
    line
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect()
}

fn left_right(grid: &[Vec<u32>]) -> Vec<Vec<bool>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut visible = vec![vec![false; width]; height];

    for i in 0..height {
        let mut max = 0;
        for j in 0..width {
            if grid[i][j] > max || j == 0 {
                max = grid[i][j];
                visible[i][j] = true;
            }
        }
    }

    visible
}

fn right_left(grid: &[Vec<u32>]) -> Vec<Vec<bool>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut visible = vec![vec![false; width]; height];

    for i in 0..height {
        let mut max = 0;
        for j in (0..width).rev() {
            if grid[i][j] > max || j == width - 1 {
                max = grid[i][j];
                visible[i][j] = true;
            }
        }
    }

    visible
}

fn top_down(grid: &[Vec<u32>]) -> Vec<Vec<bool>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut visible = vec![vec![false; width]; height];

    for j in 0..width {
        let mut max = 0;
        for i in 0..height {
            if grid[i][j] > max || i == 0 {
                max = grid[i][j];
                visible[i][j] = true;
            }
        }
    }

    visible
}

fn bottom_up(grid: &[Vec<u32>]) -> Vec<Vec<bool>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut visible = vec![vec![false; width]; height];

    for j in 0..width {
        let mut max = 0;
        for i in (0..height).rev() {
            let val = grid[i][j];
            if val > max || i == height - 1 {
                max = val;
                visible[i][j] = true;
            }
        }
    }

    visible
}

fn or(g1: Vec<Vec<bool>>, g2: Vec<Vec<bool>>, g3: Vec<Vec<bool>>, g4: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    izip!(g1, g2, g3, g4)
        .map(|(row1, row2, row3, row4)| {
            izip!(row1, row2, row3, row4)
                .map(|(b1, b2, b3, b4)| b1 || b2 || b3 || b4)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn scenic_score(i: usize, j: usize, grid: &[Vec<u32>]) -> usize {
    let height = grid[i][j];
    let up = (0..i).rev()
        .map(|i| grid[i][j])
        .take_until(|h| *h >= height)
        .count();


    let down = (i + 1..grid.len())
        .map(|i| grid[i][j])
        .take_until(|h| *h >= height)
        .count();

    let left = (0..j)
        .rev()
        .map(|j| grid[i][j])
        .take_until(|h| *h >= height)
        .count();

    let right = (j + 1..grid[i].len())
        .map(|j| grid[i][j])
        .take_until(|h| *h >= height)
        .count();

    up * down * right * left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_right() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![true, true, true], vec![true, true, true], vec![true, true, true]];
        assert_eq!(left_right(&grid), expected);

        let grid = vec![vec![3, 2, 1], vec![6, 5, 4], vec![9, 8, 7]];
        let expected = vec![vec![true, false, false], vec![true, false, false], vec![true, false, false]];
        assert_eq!(left_right(&grid), expected);

        let grid = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        let expected = vec![vec![true, false, false], vec![true, false, false], vec![true, false, false]];
        assert_eq!(left_right(&grid), expected);
    }

    #[test]
    fn test_right_left() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![false, false, true], vec![false, false, true], vec![false, false, true]];
        assert_eq!(right_left(&grid), expected);

        let grid = vec![vec![3, 2, 1], vec![6, 5, 4], vec![9, 8, 7]];
        let expected = vec![vec![true, true, true], vec![true, true, true], vec![true, true, true]];
        assert_eq!(right_left(&grid), expected);

        let grid = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        let expected = vec![vec![false, false, true], vec![false, false, true], vec![false, false, true]];
        assert_eq!(right_left(&grid), expected);
    }

    #[test]
    fn test_top_down() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![true, true, true], vec![true, true, true], vec![true, true, true]];
        assert_eq!(top_down(&grid), expected);

        let grid = vec![vec![3, 2, 1], vec![6, 5, 4], vec![9, 8, 7]];
        let expected = vec![vec![true, true, true], vec![true, true, true], vec![true, true, true]];
        assert_eq!(top_down(&grid), expected);

        let grid = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        let expected = vec![vec![true, true, true], vec![false, false, false], vec![false, false, false]];
        assert_eq!(top_down(&grid), expected);

        let grid = vec![vec![3, 1, 1], vec![2, 1, 1], vec![1, 1, 2]];
        let expected = vec![vec![true, true, true], vec![false, false, false], vec![false, false, true]];
        assert_eq!(top_down(&grid), expected);
    }

    #[test]
    fn test_bottom_up() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![false, false, false], vec![false, false, false], vec![true, true, true]];
        assert_eq!(bottom_up(&grid), expected);

        let grid = vec![vec![3, 2, 1], vec![6, 5, 4], vec![9, 8, 7]];
        let expected = vec![vec![false, false, false], vec![false, false, false], vec![true, true, true]];
        assert_eq!(bottom_up(&grid), expected);

        let grid = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        let expected = vec![vec![false, false, false], vec![false, false, false], vec![true, true, true]];
        assert_eq!(bottom_up(&grid), expected);

        let grid = vec![vec![3, 1, 1], vec![2, 1, 1], vec![1, 1, 2]];
        let expected = vec![vec![true, false, false], vec![true, false, false], vec![true, true, true]];
        assert_eq!(bottom_up(&grid), expected);
    }

    #[test]
    fn test_scenic_score() {
        let grid = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0]];
        assert_eq!(scenic_score(1, 2, &grid), 4);
        assert_eq!(scenic_score(3, 2, &grid), 8);
    }
}