use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::anyhow;

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 04 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day05.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_one(lines)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day05.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };

    println!("part two : {}", res_part_two);
    Ok(())
}

fn get_rules<I>(lines: &mut I) -> anyhow::Result<Vec<(u64, u64)>>
where
    I: Iterator<Item = String>,
{
    let mut rules = Vec::new();

    for line in lines {
        if line.is_empty() {
            break;
        }

        let index = line
            .char_indices()
            .find_map(|(i, c)| if c == '|' { Some(i) } else { None })
            .ok_or_else(|| anyhow!("failed to find separator for rule"))?;
        let (first_str, second_str) = line.split_at(index);

        let (first, second) = (first_str.parse::<u64>()?, second_str[1..].parse::<u64>()?);
        rules.push((first, second));
    }

    Ok(rules)
}

fn is_rule_matched(update: &[u64], rules: &[(u64, u64)]) -> bool {
    for (r1, r2) in rules.iter() {
        let c1 = update
            .iter()
            .enumerate()
            .find_map(|(i, r)| if r == r1 { Some(i) } else { None });
        let c2 = update
            .iter()
            .enumerate()
            .find_map(|(i, r)| if r == r2 { Some(i) } else { None });
        if c1.is_some() && c2.is_some() {
            let (c1, c2) = (c1.unwrap(), c2.unwrap());

            if c1 > c2 {
                return false;
            }
        }
    }
    true
}
fn part_one<I>(mut lines: I) -> anyhow::Result<u64>
where
    I: Iterator<Item = String>,
{
    let rules = get_rules(&mut lines)?;
    let res = lines
        .map(|update| {
            update
                .split(',')
                .filter_map(|c| c.parse::<u64>().ok())
                .collect::<Vec<_>>()
        })
        .filter_map(|update| {
            if is_rule_matched(&update, &rules) {
                Some(update)
            } else {
                None
            }
        })
        .map(|l| l[l.len() / 2])
        .sum();

    Ok(res)
}

fn part_two<I>(mut lines: I) -> anyhow::Result<u64>
where
    I: Iterator<Item = String>,
{
    let rules = get_rules(&mut lines)?;

    let res = lines
        .map(|update| {
            update
                .split(',')
                .filter_map(|c| c.parse::<u64>().ok())
                .collect::<Vec<_>>()
        })
        .filter_map(|update| {
            if is_rule_matched(&update, &rules) {
                None
            } else {
                Some(update)
            }
        })
        .map(|mut update| {
            let mut work = true;
            while work {
                work = false;
                for (r1, r2) in rules.iter() {
                    let c1 =
                        update
                            .iter()
                            .enumerate()
                            .find_map(|(i, r)| if r == r1 { Some(i) } else { None });
                    let c2 =
                        update
                            .iter()
                            .enumerate()
                            .find_map(|(i, r)| if r == r2 { Some(i) } else { None });
                    if c1.is_some() && c2.is_some() {
                        let (c1, c2) = (c1.unwrap(), c2.unwrap());

                        if c1 > c2 {
                            work = true;

                            update.swap(c1, c2);
                        }
                    }
                }
            }
            update
        })
        .map(|l| l[l.len() / 2])
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
            File::open("./inputs/day05-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(143, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day05-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(123, res);
    }
}
