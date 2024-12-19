use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 11 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day11.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_one(lines)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day11.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };

    println!("part two : {}", res_part_two);
    Ok(())
}

fn part_one<I>(mut lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let mut stones = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    for i in 0..25 {
        let mut index = 0;
        while index < stones.len() {
            let nbr_digits = stones[index].checked_ilog10().unwrap_or(0) + 1;
            if stones[index] == 0 {
                stones[index] += 1;
                index += 1;
            } else if nbr_digits % 2 == 0 {
                let div = 10u64.pow(nbr_digits / 2);
                let left = stones[index] / div;
                let right = stones[index] - left * div;

                stones[index] = left;
                stones.insert(index + 1, right);

                index += 2;
            } else {
                stones[index] *= 2024;
                index += 1;
            }
        }
    }

    Ok(stones.len())
}

#[derive(Debug)]
enum Node {
    Value {
        value: u64,
        last_update: usize,
    },
    Split {
        left: Rc<RefCell<Node>>,
        right: Rc<RefCell<Node>>,
        last_update: usize,
    },
}

impl Node {
    fn count(&self) -> u64 {
        match self {
            Node::Value { .. } => 1,
            Node::Split { left, right, .. } => left.borrow().count() + right.borrow().count(),
        }
    }

    fn run(
        thiz: &mut Rc<RefCell<Self>>,
        round: usize,
        cache: &mut HashMap<u64, Rc<RefCell<Node>>>,
    ) {
        let mut r = thiz.borrow_mut();

        println!("{:?}", thiz);
        let (left, right) = match &mut *r {
            Node::Value { value, last_update } => {
                if *last_update == round {
                    return;
                }

                *last_update += 1;

                if *value == 0 {
                    *value = 1;
                    return;
                } else {
                    let nbr_digits = value.checked_ilog10().unwrap_or(0) + 1;
                    if nbr_digits % 2 == 0 {
                        let div = 10u64.pow(nbr_digits / 2);
                        let left = *value / div;
                        let right = *value - left * div;

                        let left_node = cache
                            .entry(left)
                            .or_insert_with(|| {
                                Rc::new(RefCell::new(Node::Value {
                                    value: left,
                                    last_update: *last_update,
                                }))
                            })
                            .clone();

                        let right_node = cache
                            .entry(right)
                            .or_insert_with(|| {
                                Rc::new(RefCell::new(Node::Value {
                                    value: right,
                                    last_update: *last_update,
                                }))
                            })
                            .clone();

                        (left_node, right_node)
                    } else {
                        *value *= 2024;
                        return;
                    }
                }
            }
            Node::Split {
                left,
                right,
                last_update,
            } => {
                println!("{} {} {}", *last_update, round, cache.len());
                if *last_update == round {
                    return;
                }

                *last_update += 1;

                let mut left = left.clone();
                let mut right = right.clone();

                drop(r);

                Node::run(&mut left, round, cache);
                Node::run(&mut right, round, cache);
                return;
            }
        };
        drop(r);
        *thiz.borrow_mut() = Node::Split {
            left: left,
            right: right,
            last_update: round,
        };
    }
}

fn part_two<I>(mut lines: I) -> anyhow::Result<u64>
where
    I: Iterator<Item = String>,
{
    let mut stones = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|n| Node::Value {
            value: n.parse::<u64>().unwrap(),
            last_update: 0,
        })
        .map(|n| Rc::new(RefCell::new(n)))
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();
    for i in 1..26 {
        for n in &mut stones {
            Node::run(n, i, &mut cache);
        }
    }

    let res = stones.iter().map(|s| s.borrow().count()).sum();

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
            File::open("./inputs/day11-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(55312, res);
    }
    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day11-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(55312, res);
    }
}
