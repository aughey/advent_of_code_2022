use anyhow::Result;

fn main() -> Result<()> {
    let part1_ans = aoc::part1(aoc::part1_problem_data()?.as_str())?;
    let part2_ans = aoc::part2(aoc::part2_problem_data()?.as_str())?;

    println!("Part 1 = {part1_ans}");
    println!("Part 2 = {part2_ans}");
    Ok(())
}
