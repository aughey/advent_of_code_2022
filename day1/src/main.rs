use anyhow::Result;

fn main() -> Result<()> {
    let elves = day1::Elves(day1::read_food("problem.txt")?);

    println!(
        "{}",
        elves
            .max_calorie_elf()
            .ok_or_else(|| anyhow::anyhow!("No max elf"))?
            .total_cal()
    );

    let mut top3 = elves.sorted_by_calorie();
    top3.reverse();
    let sum : u32 = top3.into_iter().map(|e| e.total_cal()).take(3).sum();

    println!("Sum top 3 = {sum}");

    Ok(())
}
