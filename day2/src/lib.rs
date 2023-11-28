use anyhow::Result;

#[derive(Debug)]
pub struct Round {
    pub opp: Hand,
    pub me: Hand,
}
impl Round {
    pub fn score(&self) -> (u32, u32) {
        (self.opp.against(&self.me), self.me.against(&self.opp))
    }
}

#[derive(Debug, Clone)]
pub enum RoundResult {
    Win,
    Loss,
    Draw,
}
impl TryFrom<&str> for RoundResult {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let res = match value {
            "X" => RoundResult::Loss,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => anyhow::bail!("Could not parse round {value}"),
        };
        Ok(res)
    }
}

#[derive(Debug, Clone)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}
impl Hand {
    pub fn value(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
    // What hand should I play to achieve the desired result.
    // self is my opponent.
    pub fn to_achieve(&self, result: RoundResult) -> Hand {
        match (self, result) {
            (Hand::Rock, RoundResult::Win) => Hand::Paper,
            (Hand::Rock, RoundResult::Loss) => Hand::Scissors,
            (Hand::Rock, RoundResult::Draw) => Hand::Rock,
            (Hand::Paper, RoundResult::Win) => Hand::Scissors,
            (Hand::Paper, RoundResult::Loss) => Hand::Rock,
            (Hand::Paper, RoundResult::Draw) => Hand::Paper,
            (Hand::Scissors, RoundResult::Win) => Hand::Rock,
            (Hand::Scissors, RoundResult::Loss) => Hand::Paper,
            (Hand::Scissors, RoundResult::Draw) => Hand::Scissors,
        }
    }
    pub fn against(&self, other: &Hand) -> u32 {
        const WIN_SCORE: u32 = 6;
        const DRAW_SCORE: u32 = 3;
        const LOSS_SCORE: u32 = 0;

        let winloss = match (self, other) {
            (Hand::Rock, Hand::Rock) => DRAW_SCORE,
            (Hand::Rock, Hand::Paper) => LOSS_SCORE,
            (Hand::Rock, Hand::Scissors) => WIN_SCORE,
            (Hand::Paper, Hand::Rock) => WIN_SCORE,
            (Hand::Paper, Hand::Paper) => DRAW_SCORE,
            (Hand::Paper, Hand::Scissors) => LOSS_SCORE,
            (Hand::Scissors, Hand::Rock) => LOSS_SCORE,
            (Hand::Scissors, Hand::Paper) => WIN_SCORE,
            (Hand::Scissors, Hand::Scissors) => DRAW_SCORE,
        };
        winloss + self.value()
    }
}

impl TryFrom<&str> for Hand {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let res = match value {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => anyhow::bail!("could not parse hand: {value}"),
        };
        Ok(res)
    }
}

#[derive(Debug, Clone)]
pub struct Goal {
    pub opp: Hand,
    pub result: RoundResult,
}

impl TryFrom<&str> for Goal {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut values = value.split(" ");

        let (opp, result) = (
            values
                .next()
                .ok_or_else(|| anyhow::anyhow!("No opp value"))?
                .try_into()?,
            values
                .next()
                .ok_or_else(|| anyhow::anyhow!("No result value"))?
                .try_into()?,
        );
        Ok(Goal { opp, result })
    }
}

#[test]
fn test_rounds() -> Result<()> {
    let sample = include_str!("../sample.txt");
    let goals: Result<Vec<Goal>> = sample
        .lines()
        .into_iter()
        .map(|line| line.try_into())
        .collect();
    let goals = goals?;

    let rounds = goals.into_iter().map(|goal| {
        let me = goal.opp.to_achieve(goal.result);
        Round { opp: goal.opp, me }
    });

    println!("rounds: {:?}", rounds.clone().collect::<Vec<_>>());

    let scores = rounds.into_iter().map(|r| r.score());

    println!("{:?}", scores);

    let totals = scores.fold((0, 0), |(acc1, acc2), (value1, value2)| {
        ((acc1 + value1), (acc2 + value2))
    });

    println!("{:?}", totals);
    assert!(false);

    Ok(())
}
