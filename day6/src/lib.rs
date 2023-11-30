use std::collections::HashSet;

use anyhow::Result;
pub fn get_sof(data: &str) -> Result<usize> {
    get_window(data, 4)
}

pub fn get_som(data: &str) -> Result<usize> {
    get_window(data, 14)
}

pub fn get_window(data: &str, window_size: usize) -> Result<usize> {
    let mut start = 0usize;
    loop {
        let window = data
            .get(start..(start + window_size))
            .ok_or_else(|| anyhow::anyhow!("Could not find SOF"))?;
        let windowset: HashSet<_> = window.chars().collect();
        if window.len() == windowset.len() {
            return Ok(start + window_size);
        }
        start += 1;
    }
}

#[allow(unused)]
fn part1(data: &str) -> Result<String> {
    Ok(get_sof(data)?.to_string())
}

#[allow(unused)]
fn part2(data: &str) -> Result<String> {
    Ok(get_som(data)?.to_string())
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
    let expected = "10".to_string();
    let got = part1(sample_data()?.as_str())?;

    assert_eq!(expected, got);
    Ok(())
}

#[test]
#[allow(unused)]
fn test_part2() -> Result<()> {
    let expected = "29".to_string();
    let got = part2(sample_data()?.as_str())?;

    assert_eq!(expected,got);
    Ok(())
}
