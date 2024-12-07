use std::collections::{HashMap, HashSet};
use std::time::Duration;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use fxhash::{FxHashSet, FxHasher};

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 06 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day06.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_one(lines)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day06.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };

    println!("part two : {}", res_part_two);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellType {
    Empty,
    Wall,
    Guard(Direction),
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up = 0,
    Left = 1,
    Right = 2,
    Down = 3,
}

fn part_one<I>(lines: I) -> anyhow::Result<u64>
where
    I: Iterator<Item = String>,
{
    let mut map = extract_map(lines);

    // Find guard position
    let mut guard_states = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let CellType::Guard(dir) = map[y][x] {
                guard_states.push((x, y, dir));
                map[y][x] = CellType::Empty;
            }
        }
    }

    let mut nbr = 1;
    loop {
        let (last_x, last_y, last_dir) = *guard_states.last().unwrap();
        let new_state = match last_dir {
            Direction::Up => {
                if last_y == 0 {
                    break;
                } else if map[last_y - 1][last_x] != CellType::Wall {
                    (last_x, last_y - 1, last_dir)
                } else {
                    (last_x, last_y, Direction::Right)
                }
            }
            Direction::Left => {
                if last_x == 0 {
                    break;
                } else if map[last_y][last_x - 1] != CellType::Wall {
                    (last_x - 1, last_y, last_dir)
                } else {
                    (last_x, last_y, Direction::Up)
                }
            }
            Direction::Right => {
                if last_x + 1 == map[last_y].len() {
                    break;
                } else if map[last_y][last_x + 1] != CellType::Wall {
                    (last_x + 1, last_y, last_dir)
                } else {
                    (last_x, last_y, Direction::Down)
                }
            }
            Direction::Down => {
                if last_y + 1 == map.len() {
                    break;
                } else if map[last_y + 1][last_x] != CellType::Wall {
                    (last_x, last_y + 1, last_dir)
                } else {
                    (last_x, last_y, Direction::Left)
                }
            }
        };

        if !guard_states
            .iter()
            .any(|s| s.0 == new_state.0 && s.1 == new_state.1)
        {
            nbr += 1;
        }

        guard_states.push(new_state);
    }

    Ok(nbr)
}

fn extract_map<I>(lines: I) -> Vec<Vec<CellType>>
where
    I: Iterator<Item = String>,
{
    lines
        .take_while(|c| !c.is_empty())
        .map(|c| {
            c.chars()
                .map(|c| {
                    if c == '#' {
                        CellType::Wall
                    } else if c == '.' {
                        CellType::Empty
                    } else {
                        let dir = match c {
                            '^' => Direction::Up,
                            '>' => Direction::Right,
                            '<' => Direction::Left,
                            'v' => Direction::Down,
                            _ => panic!("invalid dir"),
                        };

                        CellType::Guard(dir)
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part_two<I>(lines: I) -> anyhow::Result<u64>
where
    I: Iterator<Item = String>,
{
    let mut map = extract_map(lines);

    // Find guard position
    let mut guard_states = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let CellType::Guard(dir) = map[y][x] {
                guard_states.push((x, y, dir));
                map[y][x] = CellType::Empty;
            }
        }
    }

    let possible_block_positions = get_possible_positions(&map, &guard_states);

    let state = *guard_states.last().unwrap();

    let mut guard_states_set = FxHashSet::default();

    let mut nbr = 0;
    for (x, y) in possible_block_positions.iter().skip(1) {
        let (x, y) = (*x, *y);
        if map[y][x] == CellType::Wall || (x == state.0 && y == state.1) {
            continue;
        }

        let mut map = map.iter().map(|m| m.clone()).collect::<Vec<_>>();

        let mut last_state = *guard_states.first().unwrap();

        guard_states_set.clear();
        guard_states_set.insert(last_state);

        map[y][x] = CellType::Wall;

        let stuck = loop {
            let (last_x, last_y, last_dir) = last_state;
            let new_state = match last_dir {
                Direction::Up => {
                    if last_y == 0 {
                        break false;
                    } else if map[last_y - 1][last_x] != CellType::Wall {
                        (last_x, last_y - 1, last_dir)
                    } else {
                        (last_x, last_y, Direction::Right)
                    }
                }
                Direction::Left => {
                    if last_x == 0 {
                        break false;
                    } else if map[last_y][last_x - 1] != CellType::Wall {
                        (last_x - 1, last_y, last_dir)
                    } else {
                        (last_x, last_y, Direction::Up)
                    }
                }
                Direction::Right => {
                    if last_x + 1 == map[last_y].len() {
                        break false;
                    } else if map[last_y][last_x + 1] != CellType::Wall {
                        (last_x + 1, last_y, last_dir)
                    } else {
                        (last_x, last_y, Direction::Down)
                    }
                }
                Direction::Down => {
                    if last_y + 1 == map.len() {
                        break false;
                    } else if map[last_y + 1][last_x] != CellType::Wall {
                        (last_x, last_y + 1, last_dir)
                    } else {
                        (last_x, last_y, Direction::Left)
                    }
                }
            };

            if guard_states_set.contains(&new_state) {
                break true;
            }

            last_state = new_state;

            guard_states_set.insert(new_state);
        };

        if stuck {
            nbr += 1;
        }
    }

    Ok(nbr)
}

fn get_possible_positions(
    map: &Vec<Vec<CellType>>,
    guard_states: &Vec<(usize, usize, Direction)>,
) -> Vec<(usize, usize)> {
    let mut last_state = *guard_states.first().unwrap();
    let mut guard_states = Vec::new();
    loop {
        let (last_x, last_y, last_dir) = last_state;
        let new_state = match last_dir {
            Direction::Up => {
                if last_y == 0 {
                    break;
                } else if map[last_y - 1][last_x] != CellType::Wall {
                    (last_x, last_y - 1, last_dir)
                } else {
                    (last_x, last_y, Direction::Right)
                }
            }
            Direction::Left => {
                if last_x == 0 {
                    break;
                } else if map[last_y][last_x - 1] != CellType::Wall {
                    (last_x - 1, last_y, last_dir)
                } else {
                    (last_x, last_y, Direction::Up)
                }
            }
            Direction::Right => {
                if last_x + 1 == map[last_y].len() {
                    break;
                } else if map[last_y][last_x + 1] != CellType::Wall {
                    (last_x + 1, last_y, last_dir)
                } else {
                    (last_x, last_y, Direction::Down)
                }
            }
            Direction::Down => {
                if last_y + 1 == map.len() {
                    break;
                } else if map[last_y + 1][last_x] != CellType::Wall {
                    (last_x, last_y + 1, last_dir)
                } else {
                    (last_x, last_y, Direction::Left)
                }
            }
        };

        let s = (new_state.0, new_state.1);
        if !guard_states.contains(&s) {
            guard_states.push(s);
        }
        last_state = new_state;
    }
    guard_states
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
            File::open("./inputs/day06-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(41, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day06-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(6, res);
    }
}
