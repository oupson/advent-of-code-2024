use std::collections::{HashSet, VecDeque};
use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    rc::{Rc, Weak},
};

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 12 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day12.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_one(lines)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day12.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };

    println!("part two : {}", res_part_two);
    Ok(())
}

struct Cell {
    bounds: u8,
    region_id: usize,
}

fn flow(map: &mut Vec<Vec<(char, Option<Cell>)>>, x: usize, y: usize, region_id: usize) {
    map[y][x].1 = Some(Cell {
        bounds: 0,
        region_id,
    });
    let mut bounds = 0;
    let c = map[y][x].0;

    if y > 0 {
        if map[y - 1][x].0 != c {
            bounds += 1;
        } else if map[y - 1][x].1.is_none() {
            flow(map, x, y - 1, region_id);
        }
    } else {
        bounds += 1;
    }

    if y + 1 < map.len() {
        if map[y + 1][x].0 != c {
            bounds += 1;
        } else if map[y + 1][x].1.is_none() {
            flow(map, x, y + 1, region_id);
        }
    } else {
        bounds += 1;
    }

    if x > 0 {
        if map[y][x - 1].0 != c {
            bounds += 1;
        } else if map[y][x - 1].1.is_none() {
            flow(map, x - 1, y, region_id);
        }
    } else {
        bounds += 1;
    }

    if x + 1 < map[y].len() {
        if map[y][x + 1].0 != c {
            bounds += 1;
        } else if map[y][x + 1].1.is_none() {
            flow(map, x + 1, y, region_id);
        }
    } else {
        bounds += 1;
    }

    map[y][x].1.as_mut().unwrap().bounds = bounds;
}

fn part_one<I>(lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let mut map = lines
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| (c, None::<Cell>)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut region_id = 0;

    while let Some((x, y)) = {
        map.iter().enumerate().find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, c)| if c.1.is_none() { Some(x) } else { None })
                .map(|x| (x, y))
        })
    } {
        flow(&mut map, x, y, region_id);
        region_id += 1;
    }

    let res = (0..region_id)
        .map(|i| {
            let mut area = 0;
            let mut periph = 0;
            for b in map.iter().flatten().filter_map(|(_, c)| {
                c.as_ref().and_then(|c| {
                    if c.region_id == i {
                        Some(c.bounds)
                    } else {
                        None
                    }
                })
            }) {
                area += 1;
                periph += b as usize;
            }

            area * periph
        })
        .sum();

    Ok(res)
}

struct Node {
    value: char,
    visited: bool,
    top: Option<Weak<RefCell<Node>>>,
    right: Option<Weak<RefCell<Node>>>,
    left: Option<Weak<RefCell<Node>>>,
    bottom: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(value: char) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value,
            visited: false,
            top: None,
            right: None,
            left: None,
            bottom: None,
        }))
    }

    #[inline]
    fn top_value(&self) -> Option<char> {
        self.top
            .as_ref()
            .and_then(|c| c.upgrade())
            .map(|t| t.borrow().value)
    }

    #[inline]
    fn bottom_value(&self) -> Option<char> {
        self.bottom
            .as_ref()
            .and_then(|c| c.upgrade())
            .map(|t| t.borrow().value)
    }

    #[inline]
    fn left_value(&self) -> Option<char> {
        self.left
            .as_ref()
            .and_then(|c| c.upgrade())
            .map(|t| t.borrow().value)
    }

    #[inline]
    fn right_value(&self) -> Option<char> {
        self.right
            .as_ref()
            .and_then(|c| c.upgrade())
            .map(|t| t.borrow().value)
    }
}

fn part_two<I>(lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let map = lines
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(Node::new).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if x != map[y].len() - 1 {
                map[y][x].borrow_mut().right = Some(Rc::downgrade(&map[y][x + 1]));
                map[y][x + 1].borrow_mut().left = Some(Rc::downgrade(&map[y][x]));
            }

            if y != map.len() - 1 {
                map[y][x].borrow_mut().bottom = Some(Rc::downgrade(&map[y + 1][x]));
                map[y + 1][x].borrow_mut().top = Some(Rc::downgrade(&map[y][x]));
            }
        }
    }

    let mut total = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let mut m = cell.borrow_mut();
            if !m.visited {
                m.visited = true;
                drop(m);

                let mut lefts: HashMap<usize, HashSet<usize>> = HashMap::new();
                let mut rights: HashMap<usize, HashSet<usize>> = HashMap::new();
                let mut tops: HashMap<usize, HashSet<usize>> = HashMap::new();
                let mut bottoms: HashMap<usize, HashSet<usize>> = HashMap::new();

                let mut size = 1;

                let mut queue = VecDeque::new();
                queue.push_back((x, y, map[y][x].clone()));

                while let Some((x, y, r)) = queue.pop_front() {
                    let m = r.borrow();
                    let v = Some(m.value);
                    if m.top_value() != v {
                        tops.entry(y).or_default().insert(x);
                    } else if let Some(r) = m.top.as_ref().and_then(|r| r.upgrade()) {
                        let mut c = r.borrow_mut();
                        if !c.visited {
                            size += 1;
                            c.visited = true;
                            drop(c);
                            queue.push_back((x, y - 1, r));
                        }
                    }

                    if m.bottom_value() != v {
                        bottoms.entry(y).or_default().insert(x);
                    } else if let Some(r) = m.bottom.as_ref().and_then(|r| r.upgrade()) {
                        let mut c = r.borrow_mut();
                        if !c.visited {
                            size += 1;
                            c.visited = true;
                            drop(c);
                            queue.push_back((x, y + 1, r));
                        }
                    }

                    if m.left_value() != v {
                        lefts.entry(x).or_default().insert(y);
                    } else if let Some(r) = m.left.as_ref().and_then(|r| r.upgrade()) {
                        let mut c = r.borrow_mut();
                        if !c.visited {
                            size += 1;
                            c.visited = true;
                            drop(c);
                            queue.push_back((x - 1, y, r));
                        }
                    }
                    if m.right_value() != v {
                        rights.entry(x).or_default().insert(y);
                    } else if let Some(r) = m.right.as_ref().and_then(|r| r.upgrade()) {
                        let mut c = r.borrow_mut();
                        if !c.visited {
                            size += 1;
                            c.visited = true;
                            drop(c);
                            queue.push_back((x + 1, y, r));
                        }
                    }
                }

                let res = [lefts, rights, tops, bottoms]
                    .into_iter()
                    .map(|dir| {
                        dir.into_values()
                            .map(|r| {
                                let mut r = r.into_iter().collect::<Vec<_>>();
                                r.sort();
                                r
                            })
                            .map(|r| {
                                let mut r = r.into_iter();
                                if let Some(mut previous) = r.next() {
                                    let mut count = 1;
                                    for c in r {
                                        if previous + 1 != c {
                                            count += 1;
                                        }
                                        previous = c
                                    }
                                    count
                                } else {
                                    0
                                }
                            })
                            .sum::<usize>()
                    })
                    .sum::<usize>();

                total += size * res;
            }
        }
    }

    Ok(total)
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
            File::open("./inputs/day12-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(1930, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day12-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_two");

        assert_eq!(1206, res);
    }

    #[test]
    fn part_two_small() {
        let input_file = BufReader::new(
            File::open("./inputs/day12-test2.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_two");

        assert_eq!(80, res);
    }

    #[test]
    fn part_two_small_shape_e() {
        let input_file = BufReader::new(
            File::open("./inputs/day12-test3.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_two");

        assert_eq!(236, res);
    }

    #[test]
    fn part_two_other() {
        let input_file = BufReader::new(
            File::open("./inputs/day12-test4.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_two");

        assert_eq!(368, res);
    }
}
