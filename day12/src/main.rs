use anyhow::Result;

fn main() -> Result<()> {
    let part1_ans = aocday12::part1(aocday12::data::part1_problem_data()?.as_str())?;
    let part2_ans = aocday12::part2(aocday12::data::part2_problem_data()?.as_str())?;

    println!("Part 1 = {part1_ans}");
    println!("Part 2 = {part2_ans}");
    Ok(())
}
