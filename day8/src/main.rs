use anyhow::Result;

fn main() -> Result<()> {
    println!("Part 1 = {}",aocday8::part1_main()?);
    println!("Part 2 = {}",aocday8::part2_main()?);
    Ok(())
}
