use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use std::collections::HashSet;

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 10 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day10.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_one(lines)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day10.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };

    println!("part two : {}", res_part_two);
    Ok(())
}

fn parse_input<I>(lines: I) -> Vec<Vec<u8>>
where
    I: Iterator<Item = String>,
{
    lines
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[inline]
fn get_grid_value(grid : &[Vec<u8>], x : i64, y : i64) -> u8 {
    if x == -1 || y == -1 {
        10
    } else {
        *grid.get(y as usize).and_then(|row| row.get(x as usize)).unwrap_or(&10)
    }
}

fn get_trail_score(grid: &[Vec<u8>], x: usize, y: usize, reachable : &mut  HashSet<(usize, usize)>) {
    let current = grid[y][x];
    if current != 9 {
        let x = x as i64;
        let y = y as i64;
        for (x, y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            let value = get_grid_value(grid, x, y);
            if value == current + 1 {
                 get_trail_score(grid, x as usize, y as usize, reachable);
            }
        }
    } else  {
        reachable.insert((x, y));
    }
}

fn part_one<I>(lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let input = parse_input(lines);

    let mut res = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == 0 {
                let mut reachable = HashSet::new();
                get_trail_score(&input, x, y, &mut reachable);
            res += reachable.len();
            }
        }
    }

    Ok(res)
}

fn get_trail_again(grid: &[Vec<u8>], x: usize, y: usize) -> usize {
    let current = grid[y][x];
    if current != 9 {
        let x = x as i64;
        let y = y as i64;
        let mut res = 0;
        for (x, y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            let value = get_grid_value(grid, x, y);
            if value == current + 1 {
                res += get_trail_again(grid, x as usize, y as usize);
            }
        }
        res
    } else  {
        1
    }
}

fn part_two<I>(lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let input = parse_input(lines);

    let mut res = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == 0 {
                res += get_trail_again(&input, x, y);
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
            File::open("./inputs/day10-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(36, res);
    }


    #[test]
    fn part_one_simple() {
        let input_file = BufReader::new(
            File::open("./inputs/day10-test2.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(2, res);
    }

    #[test]
    fn part_one_simple_four_trails() {
        let input_file = BufReader::new(
            File::open("./inputs/day10-test3.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(4, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day10-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_two");

        assert_eq!(81, res);
    }
}
