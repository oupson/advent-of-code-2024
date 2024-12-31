use num_rational::Rational64;
use num_traits::identities::Zero;
use regex::Regex;
use std::sync::LazyLock;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 13 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day13.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_one(lines)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day13.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };

    println!("part two : {}", res_part_two);
    Ok(())
}

// thx to https://github.com/TheAlgorithms/Rust/blob/master/src/math/gaussian_elimination.rs
pub fn gaussian_elimination(matrix: &mut [[Rational64; 3]; 2]) -> Vec<Rational64> {
    let size = matrix.len();
    assert_eq!(size, matrix[0].len() - 1);

    for i in 0..size - 1 {
        for j in i..size - 1 {
            echelon(matrix, i, j);
        }
    }

    for i in (1..size).rev() {
        eliminate(matrix, i);
    }

    // Disable cargo clippy warnings about needless range loops.
    // Checking the diagonal like this is simpler than any alternative.
    #[allow(clippy::needless_range_loop)]
    for i in 0..size {
        if matrix[i][i] == Rational64::zero() {
            println!("Infinitely many solutions");
        }
    }

    let mut result = vec![Rational64::zero(); size];
    for i in 0..size {
        result[i] = matrix[i][size] / matrix[i][i];
    }
    result
}

fn echelon(matrix: &mut [[Rational64; 3]; 2], i: usize, j: usize) {
    let size = matrix.len();
    if matrix[i][i] == Rational64::zero() {
    } else {
        let factor = matrix[j + 1][i] / matrix[i][i];
        (i..size + 1).for_each(|k| {
            matrix[j + 1][k] -= factor * matrix[i][k];
        });
    }
}

fn eliminate(matrix: &mut [[Rational64; 3]; 2], i: usize) {
    let size = matrix.len();
    if matrix[i][i] == Rational64::zero() {
    } else {
        for j in (1..i + 1).rev() {
            let factor = matrix[j - 1][i] / matrix[i][i];
            for k in (0..size + 1).rev() {
                matrix[j - 1][k] -= factor * matrix[i][k];
            }
        }
    }
}

static RE_BUTTON: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("Button [AB]: X\\+(\\d+), Y\\+(\\d+)").unwrap());

static RE_PRIZE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("Prize: X=(\\d+), Y=(\\d+)").unwrap());

#[derive(Debug)]
struct Puzzle {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Puzzle {
    fn new<I>(iter: &mut I) -> anyhow::Result<Self>
    where
        I: Iterator<Item = String>,
    {
        let a = iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("failed to get first line"))?;
        let b = iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("failed to get second line"))?;
        let res = iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("failed to get third line"))?;

        let (_, [x, y]) = RE_BUTTON
            .captures(&a)
            .ok_or_else(|| anyhow::anyhow!("wrong line"))?
            .extract();

        let a = (x.parse()?, y.parse()?);

        let (_, [x, y]) = RE_BUTTON
            .captures(&b)
            .ok_or_else(|| anyhow::anyhow!("wrong line"))?
            .extract();

        let b = (x.parse()?, y.parse()?);

        let (_, [x, y]) = RE_PRIZE
            .captures(&res)
            .ok_or_else(|| anyhow::anyhow!("wrong line"))?
            .extract();

        let prize = (x.parse()?, y.parse()?);

        Ok(Self { a, b, prize })
    }

    fn solve(&self) -> Option<(i64, i64)> {
        let mut matrixs = [
            [
                Rational64::new(self.a.0, 1),
                Rational64::new(self.b.0, 1),
                Rational64::new(self.prize.0, 1),
            ],
            [
                Rational64::new(self.a.1, 1),
                Rational64::new(self.b.1, 1),
                Rational64::new(self.prize.1, 1),
            ],
        ];

        let results = gaussian_elimination(&mut matrixs);

        if results[0].denom() == &1 && results[1].denom() == &1 {
            Some((*results[0].numer(), *results[1].numer()))
        } else {
            None
        }
    }
}

fn part_one<I>(mut lines: I) -> anyhow::Result<i64>
where
    I: Iterator<Item = String>,
{
    let mut total = 0;
    loop {
        let new_puzzle = Puzzle::new(&mut lines)?;

        if let Some((a, b)) = new_puzzle.solve() {
            total += a * 3 + b;
        }

        if lines.next().is_none() {
            break;
        }
    }

    Ok(total)
}

fn part_two<I>(mut lines: I) -> anyhow::Result<i64>
where
    I: Iterator<Item = String>,
{
    // 10000000000000
    let mut total = 0;
    loop {
        let mut new_puzzle = Puzzle::new(&mut lines)?;
        new_puzzle.prize = (
            new_puzzle.prize.0 + 10000000000000,
            new_puzzle.prize.1 + 10000000000000,
        );

        if let Some((a, b)) = new_puzzle.solve() {
            total += a * 3 + b;
        }

        if lines.next().is_none() {
            break;
        }
    }

    Ok(total)
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
            File::open("./inputs/day13-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(480, res);
    }

}
