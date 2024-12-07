use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 07 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day07.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_one(lines)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day07.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };

    println!("part two : {}", res_part_two);
    Ok(())
}

fn is_possible(res: i64, current: i64, values: &[i64]) -> bool {
    if values.is_empty() {
        return res == current;
    }

    is_possible(res, current + values[0], &values[1..])
        || is_possible(res, current * values[0], &values[1..])
}

fn part_one<I>(lines: I) -> anyhow::Result<i64>
where
    I: Iterator<Item = String>,
{
    let res = lines
        .map(|line| {
            let (res, line) = line.split_once(": ").unwrap();
            (
                res.parse::<i64>().unwrap(),
                line.split(' ')
                    .map(|c| c.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .filter_map(|(res, eq)| {
            if is_possible(res, eq[0], &eq[1..]) {
                Some(res)
            } else {
                None
            }
        })
        .sum();
    Ok(res)
}

fn is_possible2(res: i64, current: i64, values: &[i64]) -> bool {
    if values.is_empty() {
        return res == current;
    }

    is_possible2(res, current + values[0], &values[1..])
        || is_possible2(res, current * values[0], &values[1..])
        || {
            let nbr = values[0].checked_ilog10().unwrap_or(0) + 1;
            let new = current * 10i64.pow(nbr) + values[0];
            is_possible2(res, new, &values[1..])
        }
}

fn part_two<I>(lines: I) -> anyhow::Result<i64>
where
    I: Iterator<Item = String>,
{
    let res = lines
        .map(|line| {
            let (res, line) = line.split_once(": ").unwrap();
            (
                res.parse::<i64>().unwrap(),
                line.split(' ')
                    .map(|c| c.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .filter_map(|(res, eq)| {
            if is_possible2(res, eq[0], &eq[1..]) {
                Some(res)
            } else {
                None
            }
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
            File::open("./inputs/day07-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(3749, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day07-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(11387, res);
    }
}
