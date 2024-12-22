use std::{
    collections::{HashMap, HashSet},
    env::var,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::{BufRead, BufReader},
    iter::Enumerate,
    ops::{AddAssign, BitXor, Rem},
};

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 22 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day22.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_one(lines)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day22.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };
    println!("part two : {:?}", res_part_two);
    Ok(())
}

#[inline]
fn mix<A>(v: A, n: A) -> A
where
    A: BitXor<Output = A>,
{
    n.bitxor(v)
}

#[inline]
fn prune<A>(v: A) -> A
where
    A: Rem<Output = A> + From<u32>,
{
    v % (16777216.into())
}

fn part_one<I>(lines: I) -> anyhow::Result<u64>
where
    I: Iterator<Item = String>,
{
    Ok(lines
        .map(|l| {
            l.parse::<u64>()
                .map(|mut v| {
                    for _ in 0..2000 {
                        v = prune(mix(v, v * 64));
                        v = prune(mix(v, v / 32));
                        v = prune(mix(v, v * 2048));
                    }
                    v
                })
                .unwrap_or_default()
        })
        .sum())
}

#[inline]
fn calculate_hash<T: Hash + ?Sized>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

// 1538 to low
fn part_two<I>(lines: I) -> anyhow::Result<i64>
where
    I: Iterator<Item = String>,
{
    let sequences = lines
        .map(|l| {
            let mut r = (Vec::new(), Vec::new());
            if let Ok(mut v) = l.parse::<i64>() {
                let mut prec = v % 10;
                r.0.push(prec);
                r.1.push(prec);

                for _ in 0..2000 {
                    v = prune(mix(v, v * 64));
                    v = prune(mix(v, v / 32));
                    v = prune(mix(v, v * 2048));

                    let c = v % 10;

                    let diff = c - prec;

                    r.0.push(c);
                    r.1.push(diff);
                    prec = c;
                }
            }

            r
        })
        .collect::<Vec<_>>();

    let mut cache: HashMap<u64, i64> = HashMap::new();
    for (values, diff) in sequences {
        let mut set = HashSet::new();
        for i in 4..(diff.len() + 1) {
            // 0,1,2,3,4
            //         i
            //       a
            let hash_slice = calculate_hash(&diff[i - 4..i]);
            if !set.contains(&hash_slice) {
                set.insert(hash_slice);
                cache
                    .entry(hash_slice)
                    .or_default()
                    .add_assign(values[i - 1]);
            }
        }
    }

    Ok(cache.into_iter().map(|(_, v)| v).max().unwrap())
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
            File::open("./inputs/day22-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(37327623, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day22-test2.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_two");

        assert_eq!(23, res);
    }
}
