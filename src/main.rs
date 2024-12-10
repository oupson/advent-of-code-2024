mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() -> anyhow::Result<()> {
    day01::run_day()?;
    day02::run_day()?;
    day03::run_day()?;
    day04::run_day()?;
    day05::run_day()?;
    day06::run_day()?;
    day07::run_day()?;
    day08::run_day()?;
    day09::run_day()?;
    day10::run_day()?;
    Ok(())
}
