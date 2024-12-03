use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};

use regex::Regex;

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 01 ===");
    let input_file =
        BufReader::new(File::open("./inputs/day01.txt").expect("failed to open input file"));

    let lines = input_file.lines().map(|l| l.unwrap());
    let res_part_one = part_one(lines)?;

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day01.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };

    println!("part two : {}", res_part_two);

    Ok(())
}

fn part_one<I>(lines: I) -> anyhow::Result<i64>
where
    I: Iterator<Item = String>,
{
    let numbers = number_iter(lines)?;
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for (first, second) in numbers {
        first_list.push(first);
        second_list.push(second);
    }

    first_list.sort();
    second_list.sort();

    let res = zip(first_list, second_list)
        .map(|(first, second)| (first - second).abs())
        .sum();

    Ok(res)
}

fn part_two<I>(lines: I) -> anyhow::Result<i64>
where
    I: Iterator<Item = String>,
{
    let numbers = number_iter(lines)?;
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for (first, second) in numbers {
        first_list.push(first);
        second_list.push(second);
    }

    let res = first_list
        .into_iter()
        .map(|first| {
            first
                * (second_list
                    .iter()
                    .filter(|second| **second == first)
                    .count() as i64)
        })
        .sum();

    Ok(res)
}

fn number_iter<I>(lines: I) -> anyhow::Result<impl Iterator<Item = (i64, i64)>>
where
    I: Iterator<Item = String>,
{
    let line_regex = Regex::new("^(\\d+)\\s+(\\d+)$")?;

    let numbers = lines.filter_map(move |line| {
        line_regex
            .captures(&line)
            .map(|capture| {
                (
                    capture.get(1).unwrap().as_str(),
                    capture.get(2).unwrap().as_str(),
                )
            })
            .map(|(first, second)| {
                (
                    first.parse::<i64>().unwrap(),
                    second.parse::<i64>().unwrap(),
                )
            })
    });
    Ok(numbers)
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
            File::open("./inputs/day01-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(11, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day01-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(31, res);
    }
}
