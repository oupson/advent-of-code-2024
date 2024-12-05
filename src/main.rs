mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() -> anyhow::Result<()> {
    day01::run_day()?;
    day02::run_day()?;
    day03::run_day()?;
    day04::run_day()?;
    day05::run_day()?;
    Ok(())
}
