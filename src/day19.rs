use std::collections::{HashMap, VecDeque};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 19 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day19.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_one(lines)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day19.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };
    println!("part two : {:?}", res_part_two);
    Ok(())
}

fn match_patterns(s: &str, patterns: &[String]) -> bool {
    if s.is_empty() {
        return true;
    }
    for v in patterns {
        if s.starts_with(v) && match_patterns(&s[v.len()..], patterns) {
            return true;
        }
    }

    false
}

fn part_one<I>(mut lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let patterns = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let _ = lines.next();

    let mut nbr = 0;
    for l in lines {
        if match_patterns(&l, &patterns) {
            nbr += 1;
        }
    }

    Ok(nbr)
}

fn match_patterns_counting(
    s: &str,
    patterns: &[String],
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(n) = cache.get(s) {
        return *n;
    }

    if s.is_empty() {
        return 1;
    }

    let mut nbr = 0;
    for v in patterns {
        if s.starts_with(v) {
            nbr += match_patterns_counting(&s[v.len()..], patterns, cache);
        }
    }

    cache.insert(s.to_string(), nbr);

    nbr
}

fn part_two<I>(mut lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let patterns = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let _ = lines.next();

    let mut nbr = 0;
    let mut cache = HashMap::new();
    for l in lines {
        nbr += match_patterns_counting(&l, &patterns, &mut cache);
    }

    Ok(nbr)
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
            File::open("./inputs/day19-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(6, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day19-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_two");

        assert_eq!(16, res);
    }
}
