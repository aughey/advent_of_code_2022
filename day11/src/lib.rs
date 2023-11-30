use anyhow::Result;

#[allow(unused)]
fn part1(data: &str) -> Result<String> {
    Ok("NOT IMPLEMENTED".to_string())
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
    let expected = "foobar".to_string();
    let got = part1(sample_data()?.as_str())?;

    //assert_eq!(expected,got);
    Ok(())
}

#[test]
#[allow(unused)]
fn test_part2() -> Result<()> {
    let expected = "foobar".to_string();
    let got = part2(sample_data()?.as_str())?;

    //assert_eq!(expected,got);
    Ok(())
}