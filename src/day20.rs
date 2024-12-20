use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 20 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day20.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_one(lines, 100)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day20.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };
    println!("part two : {:?}", res_part_two);
    Ok(())
}

struct State {
    previous: (usize, usize),
    visited: bool,
}

struct Cell {
    cell_type: CellType,
    visited: bool,
    ptr: (usize, usize),
}

impl Cell {
    fn new(cell_type: CellType) -> Self {
        Self {
            cell_type,
            visited: false,
            ptr: (0, 0),
        }
    }

    fn is_wall(&self) -> bool {
        self.cell_type == CellType::Wall
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellType {
    Wall,
    Track(TrackType),
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        if value == '#' {
            CellType::Wall
        } else {
            CellType::Track(TrackType::from(value))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TrackType {
    Start,
    End,
    Normal,
}

impl From<char> for TrackType {
    fn from(value: char) -> Self {
        match value {
            'S' => TrackType::Start,
            'E' => TrackType::End,
            _ => TrackType::Normal,
        }
    }
}

fn normal_track(grid: &mut Vec<Vec<Cell>>) -> usize {
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, r)| {
            r.iter()
                .position(|c| c.cell_type == CellType::Track(TrackType::Start))
                .map(|x| (x, y))
        })
        .unwrap();

    let end = grid
        .iter()
        .enumerate()
        .find_map(|(y, r)| {
            r.iter()
                .position(|c| c.cell_type == CellType::Track(TrackType::End))
                .map(|x| (x, y))
        })
        .unwrap();

    let mut queue = VecDeque::new();
    grid[start.1][start.0].visited = true;
    queue.push_back(start);

    while !queue.is_empty() {
        let (xv, yv) = queue.pop_front().unwrap();
        if xv == end.0 && yv == end.1 {
            let mut n = 1;
            let mut p = grid[yv][xv].ptr;
            while p != start {
                p = grid[p.1][p.0].ptr;
                n += 1;
            }
            return n;
        } else {
            if xv > 0 && !grid[yv][xv - 1].is_wall() && !grid[yv][xv - 1].visited {
                grid[yv][xv - 1].visited = true;
                grid[yv][xv - 1].ptr = (xv, yv);
                queue.push_back((xv - 1, yv));
            }

            if xv + 1 < grid.len() && !grid[yv][xv + 1].is_wall() && !grid[yv][xv + 1].visited {
                grid[yv][xv + 1].visited = true;
                grid[yv][xv + 1].ptr = (xv, yv);
                queue.push_back((xv + 1, yv));
            }

            if yv + 1 < grid.len() && !grid[yv + 1][xv].is_wall() && !grid[yv + 1][xv].visited {
                grid[yv + 1][xv].visited = true;
                grid[yv + 1][xv].ptr = (xv, yv);
                queue.push_back((xv, yv + 1));
            }

            if yv > 0 && !grid[yv - 1][xv].is_wall() && !grid[yv - 1][xv].visited {
                grid[yv - 1][xv].visited = true;
                grid[yv - 1][xv].ptr = (xv, yv);
                queue.push_back((xv, yv - 1));
            }
        }
    }

    unimplemented!()
}

fn cheat(
    grid: &mut Vec<Vec<Cell>>,
    possible: &mut HashSet<((usize, usize), (usize, usize))>,
    pos: (usize, usize),
    end: (usize, usize),
    cheat: bool,
    current: usize,
    max: usize,
) -> usize {
    if current == max {
        0
    } else if pos == end {
        return max - current;
    } else {
        if pos.0 > 1 {
            if grid[pos.1][pos.0 - 1].is_wall()
                && !possible.contains(&((pos.0, pos.1), (pos.0, pos.1 - 1)))
            {}
        }

        0
    }
}

fn part_one<I>(mut lines: I, picosecs: usize) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let mut l = lines
        .map(|l| l.chars().map(|c| Cell::new(c.into())).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let s = normal_track(&mut l);

    println!("f : {}", s);

    unimplemented!()
}

fn part_two<I>(mut lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    unimplemented!()
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
            File::open("./inputs/day20-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()), 2)
            .expect("failed to run part_one");

        assert_eq!(14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day20-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_two");

        assert_eq!(16, res);
    }
}
