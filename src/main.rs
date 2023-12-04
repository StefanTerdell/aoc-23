use anyhow::Result;

mod day1_1;
mod day1_2;
mod day2_1;
mod day2_2;
mod day3_1;
mod day3_2;

fn main() -> Result<()> {
    day1_1::main();
    day1_2::main();
    day2_1::main()?;
    day2_2::main()?;
    day3_1::main()?;
    day3_2::main()?;

    Ok(())
}