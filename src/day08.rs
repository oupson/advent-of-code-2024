use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run_day() -> anyhow::Result<()> {
    println!("=== DAY 08 ===");

    let res_part_one = {
        let input_file =
            BufReader::new(File::open("./inputs/day08.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_one(lines)?
    };

    println!("part one : {}", res_part_one);

    let res_part_two = {
        let input_file =
            BufReader::new(File::open("./inputs/day08.txt").expect("failed to open input file"));

        let lines = input_file.lines().map(|l| l.unwrap());
        part_two(lines)?
    };

    println!("part two : {}", res_part_two);
    Ok(())
}

fn get_city_map<I>(lines: I) -> (Vec<Vec<char>>, Vec<Vec<usize>>)
where
    I: Iterator<Item = String>,
{
    let lines: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
    let antipodes = lines
        .iter()
        .map(|c| c.iter().map(|_| 0).collect())
        .collect();
    (lines, antipodes)
}

fn iter_other_antennas<'s, I>(
    city_map: I,
    x: usize,
    y: usize,
    s: char,
) -> impl Iterator<Item = (i64, i64)> + use<'s, I>
where
    I: Iterator<Item = &'s [char]>,
{
    city_map
        .enumerate()
        .filter(move |(oy, _)| *oy != y)
        .flat_map(move |(oy, row)| {
            row.iter().enumerate().find_map(|(ox, s2)| {
                if *s2 == s && ox != x && oy != y {
                    Some((ox as i64, oy as i64))
                } else {
                    None
                }
            })
        })
}

#[inline]
fn update_if_needed(antipodes: &mut [Vec<usize>], ny: i64, nx: i64) {
    if ny >= 0 && (ny as usize) < antipodes.len() && nx >= 0 && (nx as usize) < antipodes[ny as usize].len() {
        antipodes[ny as usize][nx as usize] += 1;
    }
}

fn part_one<I>(lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let (city_map, mut antipodes) = get_city_map(lines);

    for y in 0..city_map.len() {
        for x in 0..city_map[y].len() {
            let s = city_map[y][x];
            if s != '.' {
                for (ox, oy) in iter_other_antennas(city_map.iter().map(|r| r.as_slice()), x, y, s)
                {
                    let y = y as i64;
                    let x = x as i64;

                    let delta_x = ox - x;
                    let delta_y = oy - y;

                    let ny = y - delta_y;
                    let nx = x - delta_x;
                    update_if_needed(&mut antipodes, ny, nx);

                    let ny = oy + delta_y;
                    let nx = ox + delta_x;
                    update_if_needed(&mut antipodes, ny, nx);
                }
            }
        }
    }

    let res = antipodes
        .iter()
        .flat_map(|row| row.iter().filter(|n| **n > 0))
        .count();

    Ok(res)
}

fn part_two<I>(lines: I) -> anyhow::Result<usize>
where
    I: Iterator<Item = String>,
{
    let (city_map, mut antipodes) = get_city_map(lines);

    for y in 0..city_map.len() {
        for x in 0..city_map[y].len() {
            let s = city_map[y][x];
            if s != '.' {
                for (ox, oy) in iter_other_antennas(city_map.iter().map(|r| r.as_slice()), x, y, s)
                {
                    let y = y as i64;
                    let x = x as i64;

                    antipodes[oy as usize][ox as usize] += 1;

                    let delta_x = ox - x;
                    let delta_y = oy - y;

                    let mut ny = y - delta_y;
                    let mut nx = x - delta_x;

                    while ny >= 0
                        && (ny as usize) < antipodes.len()
                        && nx >= 0
                        && (nx as usize) < antipodes[ny as usize].len()
                    {
                        antipodes[ny as usize][nx as usize] += 1;
                        ny -= delta_y;
                        nx -= delta_x;
                    }

                    ny = oy + delta_y;
                    nx = ox + delta_x;
                    while ny >= 0
                        && (ny as usize) < antipodes.len()
                        && nx >= 0
                        && (nx as usize) < antipodes[ny as usize].len()
                    {
                        antipodes[ny as usize][nx as usize] += 1;
                        ny += delta_y;
                        nx += delta_x;
                    }
                }
            }
        }
    }

    let res = antipodes
        .iter()
        .flat_map(|row| row.iter().filter(|n| **n > 0))
        .count();

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
            File::open("./inputs/day08-test.txt").expect("failed to open input file"),
        );

        let res = super::part_one(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(14, res);
    }

    #[test]
    fn part_two() {
        let input_file = BufReader::new(
            File::open("./inputs/day08-test.txt").expect("failed to open input file"),
        );

        let res = super::part_two(input_file.lines().map(|l| l.unwrap()))
            .expect("failed to run part_one");

        assert_eq!(34, res);
    }
}
