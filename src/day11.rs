use std::collections::HashMap;
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
        part::<25, _>(lines)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day11.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part::<75, _>(lines)?
    };

    println!("part two : {}", res_part_two);
    Ok(())
}

fn part<const N: usize, I>(mut lines: I) -> anyhow::Result<u64>
where
    I: Iterator<Item = String>,
{
    let mut cache = HashMap::new();

    let size = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap())
        .map(|c| expand::<N>(0, c, &mut cache))
        .sum::<u64>();

    Ok(size)
}

fn expand<const N: usize>(gen: usize, stone: u64, cache: &mut HashMap<(usize, u64), u64>) -> u64 {
    if gen == N {
        1
    } else if cache.contains_key(&(gen, stone)) {
        return cache[&(gen, stone)];
    } else {
        let nbr = if stone == 0 {
            expand::<N>(gen + 1, 1, cache)
        } else {
            let nbr_digits = stone.checked_ilog10().unwrap_or(0) + 1;
            if nbr_digits % 2 == 0 {
                let div = 10u64.pow(nbr_digits / 2);
                let left = stone / div;
                let right = stone - left * div;
                expand::<N>(gen + 1, left, cache) + expand::<N>(gen + 1, right, cache)
            } else {
                expand::<N>(gen + 1, stone * 2024, cache)
            }
        };
        cache.insert((gen, stone), nbr);
        nbr
    }
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

        let res = super::part::<25, _>(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(55312, res);
    }
}
