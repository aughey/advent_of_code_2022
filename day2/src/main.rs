use anyhow::Result;
use day2::{Round, Goal};

fn main() -> Result<()> {
    let data = include_str!("../problem.txt");

    let goals: Result<Vec<Goal>> = data
        .lines()
        .into_iter()
        .map(|line| line.try_into())
        .collect();
    let goals = goals?;

    let rounds = goals.into_iter().map(|goal| {
        let me = goal.opp.to_achieve(goal.result);
        Round {
            opp: goal.opp,
            me
        }
    });

    let scores = rounds.into_iter().map(|r| r.score());

    let totals = scores.fold((0,0), |(acc1,acc2),(value1,value2)| {
        ((acc1 + value1),(acc2 + value2))
    });

    println!("{:?}",totals);

    Ok(())
}
