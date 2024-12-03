mod day01;
mod day02;
mod day03;

fn main() -> anyhow::Result<()> {
    day01::run_day()?;
    day02::run_day()?;
    day03::run_day()?;
    Ok(())
}
