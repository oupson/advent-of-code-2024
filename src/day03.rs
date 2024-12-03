use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

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

fn part_two<I>(lines: I) -> anyhow::Result<u64>
where
    I: Iterator<Item = String>,
{
    let line = lines.collect::<String>();
    let regex = Regex::new(r"(?m)mul\((\d{1,3}),(\d{1,3})\)|(don't)|(do)")?;
    let result = regex.captures_iter(&line);

    let mut res = 0;

    let mut enabled = true;
    for mat in result {
        if let Some(_) = mat.get(3) {
            enabled = false;
        } else if let Some(_) = mat.get(4) {
            enabled = true;
        } else if enabled {
            let first_parameter = mat.get(1).unwrap().as_str().parse::<u64>()?;
            let second_parameter = mat.get(2).unwrap().as_str().parse::<u64>()?;
            res += first_parameter * second_parameter;
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
