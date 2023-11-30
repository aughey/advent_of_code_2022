#![allow(unused)]
use std::collections::HashSet;

use anyhow::Result;

struct Command {
    dir: Dir,
    count: usize,
}
impl Command {
    fn move_dir(&self) -> (i64, i64) {
        self.dir.move_dir()
    }
}
impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.split(" ");
        Ok(Command {
            dir: value
                .next()
                .ok_or_else(|| anyhow::anyhow!("Could not get dir"))?
                .try_into()?,
            count: value
                .next()
                .ok_or_else(|| anyhow::anyhow!("COuld not get count"))?
                .parse()?,
        })
    }
}

enum Dir {
    L,
    R,
    U,
    D,
}
impl Dir {
    fn move_dir(&self) -> (i64, i64) {
        match self {
            Dir::L => (-1, 0),
            Dir::R => (1, 0),
            Dir::U => (0, 1),
            Dir::D => (0, -1),
        }
    }
}
impl TryFrom<&str> for Dir {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "L" => Dir::L,
            "R" => Dir::R,
            "U" => Dir::U,
            "D" => Dir::D,
            _ => anyhow::bail!("Unknown direction {value}"),
        })
    }
}

#[test]
fn test_command() -> Result<()> {
    let c: Command = "R 4".try_into()?;
    assert_eq!(c.count, 4);
    assert!(matches!(c.dir, Dir::R));
    Ok(())
}

#[allow(unused)]
fn part1(data: &str) -> Result<String> {
    let mut headpos = (0i64, 0i64);
    let mut tailpos = (0i64, 0i64);

    let mut positions = HashSet::new();
    positions.insert(tailpos);

    for line in data.lines() {
        let c: Command = line.try_into()?;
        for _ in 0..c.count {
            let prior_head = headpos;

            let movedir = c.move_dir();
            headpos = (headpos.0 + movedir.0, headpos.1 + movedir.1);

            let xdist = (headpos.0 - tailpos.0).abs();
            let ydist = (headpos.1 - tailpos.1).abs();
            let dist = xdist + ydist;
            if xdist <= 1 && ydist <= 1 {
                // adjacent, don't do anything
            } else {
                tailpos = prior_head
            }
            positions.insert(tailpos);
        }
    }

    Ok(positions.len().to_string())
}

fn distance(headpos: (i64, i64), tailpos: (i64, i64)) -> u8 {
    if headpos == tailpos {
        0
    } else if headpos.0 == tailpos.0 || headpos.1 == tailpos.1 {
        1
    } else {
        2
    }
}

#[allow(unused)]
fn part2(data: &str) -> Result<String> {
    let mut rope = vec![(0i64, 0i64); 10];

    let mut positions = HashSet::new();
    positions.insert(rope.last().unwrap().to_owned());

    for line in data.lines() {
        let c: Command = line.try_into()?;
        for _ in 0..c.count {
            // Move the first position
            let mut adj_pos = {
                let first = rope.first_mut().unwrap();

                let movedir = c.move_dir();
                *first = (first.0 + movedir.0, first.1 + movedir.1);

                *first
            };

            // Proprogate the move down the line
            for i in 1..rope.len() {
                let me = rope.get_mut(i).unwrap();

                let xdist = (adj_pos.0 - me.0).abs();
                let ydist = (adj_pos.1 - me.1).abs();
                let dist = xdist + ydist;
                if xdist <= 1 && ydist <= 1 {
                    // adjacent, don't do anything
                } else {
                    *me = (me.0 + clamp11(adj_pos.0 - me.0), me.1 + clamp11(adj_pos.1 - me.1));
                }
                adj_pos = *me;
            }
            positions.insert(rope.last().unwrap().to_owned());
        }
    }

    Ok(positions.len().to_string())
}

fn clamp11(value: i64) -> i64 {
    if value > 1 {
        1
    } else if value < -1 {
        -1
    } else {
        value
    }
}

#[cfg(test)]
fn sample_data() -> Result<String> {
    Ok(std::fs::read_to_string("sample.txt")?)
}

fn part1_problem_data() -> Result<String> {
    Ok(std::fs::read_to_string("problem.txt")?)
}

fn part2_problem_data() -> Result<String> {
    Ok(std::fs::read_to_string("problem.txt")?)
}

pub fn part1_main() -> Result<String> {
    part1(part1_problem_data()?.as_str())
}

pub fn part2_main() -> Result<String> {
    part2(part2_problem_data()?.as_str())
}

#[test]
#[allow(unused)]
fn test_part1() -> Result<()> {
    let expected = "13".to_string();
    let got = part1(sample_data()?.as_str())?;

    assert_eq!(expected, got);
    Ok(())
}

#[test]
#[allow(unused)]
fn test_part2() -> Result<()> {
    let expected = "36".to_string();
    let got = part2(std::fs::read_to_string("sample2.txt")?.as_str())?;

    assert_eq!(expected, got);
    Ok(())
}
