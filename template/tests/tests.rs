use anyhow::Result;

#[test]
#[allow(unused)]
fn test_part1() -> Result<()> {
    let expected = "foobar".to_string();
    let got = aoc::part1(aoc::sample_data()?.as_str())?;

    //assert_eq!(expected,got);
    Ok(())
}

#[test]
#[allow(unused)]
fn test_part2() -> Result<()> {
    let expected = "foobar".to_string();
    let got = aoc::part2(aoc::sample_data()?.as_str())?;

    //assert_eq!(expected,got);
    Ok(())
}