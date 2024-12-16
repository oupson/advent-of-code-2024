use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 12 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day12.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_one(lines)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day12.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };

    println!("part two : {}", res_part_two);
    Ok(())
}

struct Cell {
    bounds: u8,
    region_id: usize,
}

fn flow(map: &mut Vec<Vec<(char, Option<Cell>)>>, x: usize, y: usize, region_id: usize) {
    map[y][x].1 = Some(Cell {
        bounds: 0,
        region_id,
    });
    let mut bounds = 0;
    let c = map[y][x].0;

    if y > 0 {
        if map[y - 1][x].0 != c {
            bounds += 1;
        } else if map[y - 1][x].1.is_none() {
            flow(map, x, y - 1, region_id);
        }
    } else {
        bounds += 1;
    }

    if y + 1 < map.len() {
        if map[y + 1][x].0 != c {
            bounds += 1;
        } else if map[y + 1][x].1.is_none() {
            flow(map, x, y + 1, region_id);
        }
    } else {
        bounds += 1;
    }

    if x > 0 {
        if map[y][x - 1].0 != c {
            bounds += 1;
        } else if map[y][x - 1].1.is_none() {
            flow(map, x - 1, y, region_id);
        }
    } else {
        bounds += 1;
    }

    if x + 1 < map[y].len() {
        if map[y][x + 1].0 != c {
            bounds += 1;
        } else if map[y][x + 1].1.is_none() {
            flow(map, x + 1, y, region_id);
        }
    } else {
        bounds += 1;
    }

    map[y][x].1.as_mut().unwrap().bounds = bounds;
}

fn part_one<I>(lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let mut map = lines
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| (c, None::<Cell>)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut region_id = 0;

    while let Some((x, y)) = {
        map.iter().enumerate().find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, c)| if c.1.is_none() { Some(x) } else { None })
                .map(|x| (x, y))
        })
    } {
        flow(&mut map, x, y, region_id);
        region_id += 1;
    }

    let res = (0..region_id)
        .map(|i| {
            let mut area = 0;
            let mut periph = 0;
            for b in map.iter().flatten().filter_map(|(_, c)| {
                c.as_ref().and_then(|c| {
                    if c.region_id == i {
                        Some(c.bounds)
                    } else {
                        None
                    }
                })
            }) {
                area += 1;
                periph += b as usize;
            }

            area * periph
        })
        .sum();

    Ok(res)
}
fn part_two<I>(lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    unimplemented!()
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
            File::open("./inputs/day12-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(1930, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day12-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_two");

        assert_eq!(81, res);
    }
}
