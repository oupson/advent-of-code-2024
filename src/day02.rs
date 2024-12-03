use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 02 ===");
    let input_file =
        BufReader::new(File::open("./inputs/day02.txt").expect("failed to open input file"));

    let lines = input_file.lines().map(|l| l.unwrap());
    let res_part_one = part_one(lines)?;

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day02.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };

    println!("part two : {}", res_part_two);

    Ok(())
}

fn is_safe<I>(mut line: I) -> bool
where
    I: Iterator<Item = i64>,
{
    let first = line.next().unwrap();
    let mut prev = line.next().unwrap();

    let diff = (prev - first).abs();
    if diff > 0 && diff < 4 {
        let is_decreasing = first > prev;

        while let Some(next) = line.next() {
            let diff = (next - prev).abs();
            if diff > 0 && diff < 4 {
                if next > prev && is_decreasing || next < prev && !is_decreasing {
                    return false;
                }
                prev = next;
            } else {
                return false;
            }
        }
        true
    } else {
        false
    }
}

fn part_one<I>(lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let res = lines
        .filter_map(|l| {
            let op = l.split(' ').map(|n| n.parse::<i64>().unwrap());
            if is_safe(op) {
                Some(())
            } else {
                None
            }
        })
        .count();

    Ok(res)
}

fn is_safe_nice<I>(mut line: I) -> bool
where
    I: Iterator<Item = i64>,
{
    let first = line.next().unwrap();
    let mut prev = line.next().unwrap();

    let mut fault = false;

    let diff = (prev - first).abs();
    if diff > 0 && diff < 4 {
        let is_decreasing = first > prev;

        while let Some(next) = line.next() {
            let diff = (next - prev).abs();
            if diff > 0 && diff < 4 {
                if next > prev && is_decreasing || next < prev && !is_decreasing {
                    if fault {
                        return false;
                    } else {
                        fault = true;
                        continue;
                    }
                }
                prev = next;
            } else {
                if fault {
                    return false;
                } else {
                    fault = true;
                    continue;
                }
            }
        }
        true
    } else {
        false
    }
}

fn part_two<I>(lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let res = lines
        .filter_map(|l| {
            let op = l.split(' ').map(|n| n.parse::<i64>().unwrap());
            if is_safe_nice(op) {
                Some(())
            } else {
                None
            }
        })
        .count();

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
            File::open("./inputs/day02-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(2, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day02-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(4, res);
    }
}
