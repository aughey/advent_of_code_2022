use anyhow::Result;

fn main() -> Result<()> {
    println!("Part 1 = {}",aocday10::part1_main()?);
    println!("Part 2 = {}",aocday10::part2_main()?);
    Ok(())
}
