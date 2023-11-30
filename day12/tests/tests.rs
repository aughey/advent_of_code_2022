use anyhow::Result;

#[test]
#[allow(unused)]
fn test_part1() -> Result<()> {
    let expected = "31".to_string();
    let got = aocday12::part1(aocday12::data::sample_data()?.as_str())?;

    assert_eq!(expected,got);
    Ok(())
}

#[test]
#[allow(unused)]
fn test_part2() -> Result<()> {
    let expected = "29".to_string();
    let got = aocday12::part2(aocday12::data::sample_data()?.as_str())?;

    assert_eq!(expected,got);
    Ok(())
}