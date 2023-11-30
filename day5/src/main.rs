use anyhow::Result;

fn main() -> Result<()> {
    println!("Part 1 = {}",aocday5::part1_main()?);
    println!("Part 2 = {}",aocday5::part2_main()?);
    Ok(())
}
