use std::ops::RangeInclusive;

use anyhow::Result;

pub struct ElfPair {
    elf1: ElfRange,
    elf2: ElfRange,
}
impl TryFrom<&str> for ElfPair {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut pairs = value.split(',');
        Ok(Self {
            elf1: pairs
                .next()
                .ok_or_else(|| anyhow::anyhow!("Could not get elf 1"))?
                .try_into()?,
            elf2: pairs
                .next()
                .ok_or_else(|| anyhow::anyhow!("Could not get elf 1"))?
                .try_into()?,
        })
    }
}
impl ElfPair {
    pub fn fully_overlaps(&self) -> bool {
        self.elf1
            .range
            .clone()
            .all(|v| self.elf2.range.contains(&v))
            || self
                .elf2
                .range
                .clone()
                .all(|v| self.elf1.range.contains(&v))
    }
}

pub struct ElfRange {
    pub range: RangeInclusive<u32>,
}
impl TryFrom<&str> for ElfRange {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.split("-");
        let from = value
            .next()
            .ok_or_else(|| anyhow::anyhow!("No from value"))?
            .parse()?;
        let to = value
            .next()
            .ok_or_else(|| anyhow::anyhow!("No to value"))?
            .parse()?;

        let range = from..=to;
        Ok(ElfRange { range })
    }
}

#[test]
fn test_import_elfrange() {
    let elf: ElfRange = "2-4".try_into().unwrap();
    assert_eq!(elf.range, 2..=4);
    assert_eq!(elf.range.count(),3);
}

#[test]
fn test_import_elfpair() {
    let pair: ElfPair = "2-4,6-8".try_into().unwrap();
    assert_eq!(pair.elf1.range, 2..=4);
    assert_eq!(pair.elf2.range, 6..=8);
}

#[allow(unused)]
fn part1(data: &str) -> Result<String> {
    let pairs: Result<Vec<ElfPair>> = data.lines().map(|l| l.try_into()).collect();
    let pairs = pairs?;
    Ok(pairs
        .into_iter()
        .filter(|p| p.fully_overlaps())
        .count()
        .to_string())
}

#[allow(unused)]
fn part2(data: &str) -> Result<String> {
    Ok("NOT IMPLEMENTED".to_string())
}

#[cfg(test)]
fn sample_data() -> Result<String> {
    Ok(std::fs::read_to_string("sample.txt")?)
}

fn part1_problem_data() -> Result<String> {
    Ok(std::fs::read_to_string("problem.txt")?)
}

fn part2_problem_data() -> Result<String> {
    Ok(std::fs::read_to_string("problem.txt")?)
}

pub fn part1_main() -> Result<String> {
    part1(part1_problem_data()?.as_str())
}

pub fn part2_main() -> Result<String> {
    part2(part2_problem_data()?.as_str())
}

#[test]
#[allow(unused)]
fn test_part1() -> Result<()> {
    let expected = "2".to_string();
    let got = part1(sample_data()?.as_str())?;

    assert_eq!(expected, got);
    Ok(())
}

#[test]
#[allow(unused)]
fn test_part2() -> Result<()> {
    let expected = "foobar".to_string();
    let got = part2(sample_data()?.as_str())?;

    //assert_eq!(expected, got);
    Ok(())
}
