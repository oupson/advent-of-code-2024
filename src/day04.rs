use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 04 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day04.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_one(lines)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day04.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };

    println!("part two : {}", res_part_two);
    Ok(())
}
fn part_one<I>(lines: I) -> anyhow::Result<u64>
where
    I: Iterator<Item = String>,
{
    let mut grid = lines
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut res = 0;

    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if check_down(&mut grid, x, y) {
                res += 1;
            }
            if check_right(&mut grid, x, y) {
                res += 1;
            }
            if check_diag_bottom(&mut grid, x, y) {
                res += 1;
            }
            if check_diag_top(&mut grid, x, y) {
                res += 1;
            }
        }
    }

    Ok(res)
}

fn check_down(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let end = min(grid.len(), y + 4) - y;
    if end < 4 {
        false
    } else {
        let other = (0..end).map(|e| grid[y + e][x]).collect::<Vec<_>>();

        return other == ['X', 'M', 'A', 'S'] || other == ['S', 'A', 'M', 'X'];
    }
}

fn check_right(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let end = min(grid[y].len(), x + 4) - x;
    if end < 4 {
        false
    } else {
        let other = (0..end).map(|e| grid[y][x + e]).collect::<Vec<_>>();
        return other == ['X', 'M', 'A', 'S'] || other == ['S', 'A', 'M', 'X'];
    }
}

fn check_diag_bottom(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let end = min(min(grid[y].len(), x + 4) - x, min(grid.len(), y + 4) - y);

    if end < 4 {
        false
    } else {
        let other = (0..end).map(|e| grid[y + e][x + e]).collect::<Vec<_>>();
        return other == ['X', 'M', 'A', 'S'] || other == ['S', 'A', 'M', 'X'];
    }
}

fn check_diag_top(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if y < 3 {
        return false;
    }
    let end = min(min(grid[y].len(), x + 4) - x, y + 4);

    if end < 4 {
        false
    } else {
        let other = (0..end).map(|e| grid[y - e][x + e]).collect::<Vec<_>>();

        return other == ['X', 'M', 'A', 'S'] || other == ['S', 'A', 'M', 'X'];
    }
}

fn part_two<I>(lines: I) -> anyhow::Result<u64>
where
    I: Iterator<Item = String>,
{
    let grid = lines
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut res = 0;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            let mut is_good = grid[y][x] == 'A';
            is_good = is_good
                && ((grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S')
                    || (grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M'));

            is_good = is_good
                && ((grid[y - 1][x + 1] == 'M' && grid[y + 1][x - 1] == 'S')
                    || (grid[y - 1][x + 1] == 'S' && grid[y + 1][x - 1] == 'M'));

            if is_good {
                res += 1;
            }
        }
    }

    Ok(res)
}

#[cfg(test)]
mod test {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    #[test]
    fn part_one() {
        let input_file = BufReader::new(
            File::open("./inputs/day04-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(18, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day04-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(9, res);
    }
}
