use anyhow::Result;

pub fn sample_data() -> Result<String> {
    Ok(include_str!("../sample.txt").to_string())
}

pub fn part1_problem_data() -> Result<String> {
    Ok(include_str!("../problem.txt").to_string())
}

pub fn part2_problem_data() -> Result<String> {
    Ok(include_str!("../problem.txt").to_string())
}
