use std::collections::VecDeque;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 12 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day18.txt").expect("failed to open input file"));

        let lines = input_file.lines().take(1024).map(|l| l.unwrap());
        part_one(lines, 71)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day18.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines, 71)?
    };

    println!("part two : {:?}", res_part_two);
    Ok(())
}

fn bfs(grid: &mut [Vec<(bool, bool, (usize, usize))>]) -> Option<usize> {
    let mut queue = VecDeque::new();
    grid[0][0].1 = true;

    queue.push_back((0, 0));

    while !queue.is_empty() {
        let (xv, yv) = queue.pop_front().unwrap();
        if xv == grid.len() - 1 && yv == grid.len() - 1 {
            let mut n = 1;
            let mut p = grid[yv][xv].2;
            while p.0 != 0 || p.1 != 0 {
                p = grid[p.1][p.0].2;
                n += 1;
            }
            return Some(n);
        } else {
            if xv > 0 && !grid[yv][xv - 1].0 && !grid[yv][xv - 1].1 {
                grid[yv][xv - 1].1 = true;
                grid[yv][xv - 1].2 = (xv, yv);
                queue.push_back((xv - 1, yv));
            }

            if xv + 1 < grid.len() && !grid[yv][xv + 1].0 && !grid[yv][xv + 1].1 {
                grid[yv][xv + 1].1 = true;
                grid[yv][xv + 1].2 = (xv, yv);
                queue.push_back((xv + 1, yv));
            }

            if yv + 1 < grid.len() && !grid[yv + 1][xv].0 && !grid[yv + 1][xv].1 {
                grid[yv + 1][xv].1 = true;
                grid[yv + 1][xv].2 = (xv, yv);
                queue.push_back((xv, yv + 1));
            }

            if yv > 0 && !grid[yv - 1][xv].0 && !grid[yv - 1][xv].1 {
                grid[yv - 1][xv].1 = true;
                grid[yv - 1][xv].2 = (xv, yv);
                queue.push_back((xv, yv - 1));
            }
        }
    }
    None
}

fn part_one<I>(lines: I, size: usize) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let mut grid = (0..size)
        .map(|_| vec![(false, false, (0, 0)); size])
        .collect::<Vec<_>>();

    for line in lines {
        let (x, y) = line.split_once(",").unwrap();
        let (x, y) = (x.parse::<usize>()?, y.parse::<usize>()?);
        grid[y][x].0 = true;
    }
    Ok(bfs(&mut grid).unwrap())
}
fn part_two<I>(lines: I, size: usize) -> anyhow::Result<(usize, usize)>
where
    I: Iterator<Item = String>,
{
    let mut grid = (0..size)
        .map(|_| vec![(false, false, (0, 0)); size])
        .collect::<Vec<_>>();

    for line in lines {
        let (x, y) = line.split_once(",").unwrap();
        let (x, y) = (x.parse::<usize>()?, y.parse::<usize>()?);
        grid[y][x].0 = true;

        for x in 0..size {
            for y in 0..size {
                grid[y][x].1 = false;
            }
        }

        if bfs(&mut grid).is_none() {
            return Ok((x, y));
        }
    }

    anyhow::bail!("failed to find solution for part two");
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
            File::open("./inputs/day18-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()).take(12), 7)
            .expect("failed to run part_one");

        assert_eq!(22, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day18-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()), 7)
            .expect("failed to run part_two");

        assert_eq!((6, 1), res);
    }
}
