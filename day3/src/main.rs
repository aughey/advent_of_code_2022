use anyhow::Result;
use day3::{Rucksack, part2};

fn main() -> Result<()> {
    let data = include_str!("../problem.txt");
    let rucksacks: Result<Vec<Rucksack>> = data.lines().map(|line| line.try_into()).collect();
    let rucksacks = rucksacks?;

    let sum: u64 = rucksacks
        .into_iter()
        .map(|rs| rs.in_both().unwrap().priority as u64)
        .sum();

    println!("{sum}");

    println!("Part2: {}",part2("problem.txt")?);

    Ok(())
}
