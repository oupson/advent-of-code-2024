use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::{Captures, Regex};

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 02 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day03.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_one(lines)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day03.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };

    println!("part two : {}", res_part_two);
    Ok(())
}

fn part_one<I>(mut lines: I) -> anyhow::Result<u64>
where
    I: Iterator<Item = String>,
{
    let line = lines.collect::<String>();
    let regex = Regex::new(r"(?m)mul\((\d{1,3}),(\d{1,3})\)")?;

    let result = regex.captures_iter(&line);

    let mut res = 0;

    for mat in result {
        let first_parameter = mat.get(1).unwrap().as_str().parse::<u64>()?;
        let second_parameter = mat.get(2).unwrap().as_str().parse::<u64>()?;
        res += first_parameter * second_parameter;
    }
    Ok(res)
}

enum Op {
    Mul(u64, u64),
    Enable,
    Disable,
}

impl TryFrom<Captures<'_>> for Op {
    type Error = anyhow::Error;

    fn try_from(mat: Captures<'_>) -> Result<Self, Self::Error> {
        if let Some(_) = mat.get(3) {
            Ok(Op::Disable)
        } else if let Some(_) = mat.get(4) {
            Ok(Op::Enable)
        } else {
            let first = mat
                .get(1)
                .ok_or_else(|| anyhow::format_err!("missing first operand"))?
                .as_str()
                .parse::<u64>()?;
            let second = mat
                .get(2)
                .ok_or_else(|| anyhow::format_err!("missing second operand"))?
                .as_str()
                .parse::<u64>()?;
            Ok(Op::Mul(first, second))
        }
    }
}

fn part_two<I>(lines: I) -> anyhow::Result<u64>
where
    I: Iterator<Item = String>,
{
    let regex = Regex::new(r"(?m)mul\((\d{1,3}),(\d{1,3})\)|(don't)|(do)")?;

    let res = regex
        .captures_iter(&lines.collect::<String>())
        .map(|m| Op::try_from(m))
        .flatten()
        .scan(true, |mul_enabled, op| {
            match op {
                Op::Mul(a, b) if *mul_enabled => return Some(a * b),
                Op::Enable => *mul_enabled = true,
                Op::Disable => *mul_enabled = false,
                _ => (),
            }
            return Some(0);
        })
        .sum();

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
            File::open("./inputs/day03-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(161, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day03-02-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(48, res);
    }
}
