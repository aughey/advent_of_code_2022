#![allow(unused)]
use std::{cell::{Cell, RefCell}, borrow::BorrowMut};

use anyhow::Result;

struct CPU {
    x: i64,
    cycle: u64
}
impl CPU {
    fn new() -> Self {
        Self {
            x: 1,
            cycle: 1
        }
    }
    fn eval(&mut self, instruction: &Instruction, on_cycle: impl Fn(u64,i64)) {
        match instruction {
            Instruction::Noop => {
                self.cycle += 1;
                on_cycle(self.cycle,self.x);
            },
            Instruction::Addx(amount) => {
                self.cycle += 1;
                on_cycle(self.cycle,self.x);
                self.cycle += 1;
                on_cycle(self.cycle,self.x);
                self.x += amount;
            }
        }
    }
}

enum Instruction {
    Noop,
    Addx(i64),
}
impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.split(" ");
        let op = value.next().ok_or_else(||anyhow::anyhow!("no op"))?;
        Ok(match op {
            "noop" => Instruction::Noop,
            "addx" => {
                let amount = value.next().ok_or_else(||anyhow::anyhow!("no amount"))?;
                let amount = amount.parse::<i64>()?;
                Instruction::Addx(amount)
            },
            _ => anyhow::bail!("unknown op {}",op)
        })
    }
    
}

#[allow(unused)]
fn part1(data: &str) -> Result<String> {
    let mut sum = RefCell::new(0i64);
    let mut cpu = CPU::new();

    let check_cycles = [20,60,100,140,180,220];

    for line in data.lines() {
        let inst : Instruction = line.try_into()?;
        cpu.eval(&inst,|cycle,x| {
            if check_cycles.contains(&cycle) {
                *sum.borrow_mut() += x * (cycle as i64);
            }
        });
    }
    Ok(sum.into_inner().to_string())
}

#[allow(unused)]
fn part2(data: &str) -> Result<String> {
    let crt = vec![vec![]];

    let mut crt = RefCell::new(crt);
    let mut cpu = CPU::new();

    let check_cycles = [20,60,100,140,180,220];

    for line in data.lines() {
        let inst : Instruction = line.try_into()?;
        cpu.eval(&inst,|cycle,x| {
            let mut crt = crt.borrow_mut();
            let row = crt.last_mut().unwrap();
            let row = if row.len() >= 40 {
                crt.push(vec![]);
                crt.last_mut().unwrap()
            } else {
                row
            };

            let dist = row.len() as i64  - x;
            let dist = dist.abs();
            row.push(if dist <= 1 {
                '#' 
            } else {
                '.'
            });
        });
    }

    for row in crt.borrow().iter() {
        println!("{}",row.iter().collect::<String>());
    }
    Ok("".to_string())
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
    let expected = "13140".to_string();
    let got = part1(sample_data()?.as_str())?;

    assert_eq!(expected,got);
    Ok(())
}

#[test]
#[allow(unused)]
fn test_part2() -> Result<()> {
    let expected = "foobar".to_string();
    let got = part2(sample_data()?.as_str())?;

    assert_eq!(expected,got);
    Ok(())
}