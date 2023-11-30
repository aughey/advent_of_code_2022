use anyhow::Result;

fn main() -> Result<()> {
    println!("Part 1 = {}",aocday9::part1_main()?);
    println!("Part 2 = {}",aocday9::part2_main()?);
    Ok(())
}
