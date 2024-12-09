use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 09 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day09.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_one(lines)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day09.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };

    println!("part two : {}", res_part_two);
    Ok(())
}

fn is_sorted(l: &[Option<usize>]) -> bool {
    let mut iter = l.iter();
    for s in iter.by_ref() {
        if s.is_none() {
            break;
        }
    }

    for s in iter {
        if s.is_some() {
            return false;
        }
    }

    true
}

fn part_one<I>(mut lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let input = lines.next().unwrap();

    let mut a = input
        .chars()
        .scan((true, 0), |(s, n), c| {
            let state = *s;
            *s = !state;
            let size = c.to_digit(10).unwrap() as usize;
            let res: Vec<_> = if state {
                let nbr = *n;
                if state {
                    *n += 1;
                }
                (0..size).map(|_| Some(nbr)).collect()
            } else {
                (0..size).map(|_| None).collect()
            };
            Some(res)
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut index = a.len() - 1;
    let mut last_edited = 0;
    while !is_sorted(&a) && index > last_edited {
        let item = a[index].take();
        if let Some(item) = item {
            let first_index =
                a[last_edited..].iter().position(|&x| x.is_none()).unwrap() + last_edited;
            a[first_index] = Some(item);
            last_edited = first_index;
        }
        index -= 1;
    }

    let res = a
        .iter()
        .enumerate()
        .filter_map(|(i, v)| v.map(|v| i * v))
        .sum::<usize>();
    Ok(res)
}

#[derive(Debug)]
enum Segment {
    Empty(usize),
    File { id: usize, size: usize },
}

impl Segment {
    fn size(&self) -> usize {
        match self {
            Segment::Empty(u) => *u,
            Self::File { size, .. } => *size,
        }
    }
}

fn part_two<I>(mut lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let input = lines.next().unwrap();
    let mut a = input
        .chars()
        .scan((true, 0), |(s, n), c| {
            let state = *s;
            *s = !state;
            let size = c.to_digit(10).unwrap() as usize;
            let res = if state {
                let nbr = *n;
                if state {
                    *n += 1;
                }
                Segment::File { id: nbr, size }
            } else {
                Segment::Empty(size)
            };
            Some(res)
        })
        .collect::<Vec<_>>();

    let max_id = a
        .iter()
        .filter_map(|f| {
            if let Segment::File { id, .. } = f {
                Some(*id)
            } else {
                None
            }
        })
        .max()
        .unwrap();

    let mut current_id = max_id + 1;
    while current_id > 0 {
        current_id -= 1;
        let (pos, needed_size) = a
            .iter()
            .enumerate()
            .find_map(|(i, s)| {
                if let Segment::File { id, size } = s {
                    if *id == current_id {
                        Some((i, *size))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap();

        let to_change = a[0..pos].iter().position(|s| {
            if let Segment::Empty(u) = s {
                *u >= needed_size
            } else {
                false
            }
        });

        if let Some(to_change) = to_change {
            let s = std::mem::replace(&mut a[pos], Segment::Empty(needed_size));
            let empty_space = a[to_change].size();
            a[to_change] = s;
            if empty_space - needed_size > 0 {
                a.insert(to_change + 1, Segment::Empty(empty_space - needed_size));
            }
        }
    }

    let res = a.into_iter().scan(0, |state, s| match s {
        Segment::Empty(u) => {
            *state += u;
            Some(0)
        }
        Segment::File { id, size } => {
            let mut r = 0;
            for i in *state..*state + size {
                r += i * id;
            }
            *state += size;
            Some(r)
        }
    }).sum();

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
            File::open("./inputs/day09-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(1928, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day09-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_two");

        assert_eq!(2858, res);
    }
}
