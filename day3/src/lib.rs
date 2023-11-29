use std::collections::HashSet;

use anyhow::Result;

#[derive(Debug)]
pub struct Compartment {
    pub items: Vec<Item>,
}
impl Compartment {
    pub fn total_priority(&self) -> u64 {
        self.items.iter().map(|i| i.priority as u64).sum()
    }
    pub fn to_set(&self) -> HashSet<&Item> {
        self.items.iter().collect()
    }
}

#[derive(Debug)]
pub struct Rucksack {
    pub first: Compartment,
    pub second: Compartment,
}
impl Rucksack {
    pub fn in_both(&self) -> Option<Item> {
        let first = self.first.to_set();
        let second = self.second.to_set();
        let mut intersection = first.intersection(&second);

        let value = intersection.next();

        value.map(|v| *v.to_owned())
    }
    pub fn all_items(&self) -> impl Iterator<Item = &Item> {
        self.first.items.iter().chain(self.second.items.iter())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Item {
    pub letter: char,
    pub priority: u8,
}
impl Item {
    pub fn new(c: char) -> Result<Self> {
        let value: u8 = c.try_into()?;
        let a_value: u8 = 'a'.try_into()?;
        let z_value: u8 = 'z'.try_into()?;
        let a2_value: u8 = 'A'.try_into()?;
        let z2_value: u8 = 'Z'.try_into()?;
        if value >= a_value && value <= z_value {
            Ok(Item {
                letter: c,
                priority: 1 + value - a_value,
            })
        } else if value >= a2_value && value <= z2_value {
            Ok(Item {
                letter: c,
                priority: 27 + value - a2_value,
            })
        } else {
            anyhow::bail!("out of range character {c}")
        }
    }
}

impl TryFrom<&str> for Compartment {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let items: Result<Vec<Item>> = value.chars().map(|c| Item::new(c)).collect();
        let items = items?;
        Ok(Compartment { items })
    }
}

impl TryFrom<&str> for Rucksack {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let count = value.len();
        let (near, far) = value.split_at(count / 2);
        if near.len() != far.len() {
            anyhow::bail!("Two compartments are not of equal length");
        }
        Ok(Rucksack {
            first: near.try_into()?,
            second: far.try_into()?,
        })
    }
}

#[test]
fn test_item() {
    let a = Item::new('a').unwrap();
    assert_eq!(a.letter, 'a');
    assert_eq!(a.priority, 1);

    let a = Item::new('z').unwrap();
    assert_eq!(a.letter, 'z');
    assert_eq!(a.priority, 26);

    let a = Item::new('A').unwrap();
    assert_eq!(a.letter, 'A');
    assert_eq!(a.priority, 27);

    let a = Item::new('Z').unwrap();
    assert_eq!(a.letter, 'Z');
    assert_eq!(a.priority, 27 + 25);

    let fail = Item::new('=');
    assert!(fail.is_err());
}

#[test]
fn test_sample() -> Result<()> {
    let sample = include_str!("../sample.txt");
    for line in sample.lines() {
        let rs: Rucksack = line.try_into()?;
        println!("{rs:?}");
        let dup = rs.in_both().unwrap();
        println!("{dup:?}");
    }
    Ok(())
}

fn groups_of_three(mut rucksacks: impl Iterator<Item=Rucksack>) -> impl Iterator<Item=[Rucksack;3]> {
    let group_3 = move || {
        let three: [Rucksack; 3] = [
            rucksacks.next()?,
            rucksacks.next()?,
            rucksacks.next()?,
        ];
        Some(three)
    };
    std::iter::from_fn(group_3)
}

fn common_item(three: [Rucksack;3]) -> Item {
    let all0: HashSet<_> = three[0].all_items().collect();
    let all1: HashSet<_> = three[1].all_items().collect();
    let all2: HashSet<_> = three[2].all_items().collect();
    let diff0: HashSet<_> = all0.intersection(&all1).map(|v| *v).collect();
    let mut diff1 = diff0.intersection(&all2).map(|v| *v);
    let common = diff1.next().unwrap();
    common.to_owned()
}

pub fn part2(filename: &str) -> Result<String> {
    let data = std::fs::read_to_string(filename)?;
    let lines = data.lines().into_iter();
    let rucksacks = lines.map(|l| l.try_into().unwrap());
    let groups = groups_of_three(rucksacks);

    let missing = groups.map(common_item);

    let sum: u64 = missing.map(|m| m.priority as u64).sum();
    Ok(format!("{sum}"))
}

#[test]
fn test_grouping() -> Result<()> {
    let sum = part2("sample.txt")?;

    assert_eq!(sum,"70");

    Ok(())
}
