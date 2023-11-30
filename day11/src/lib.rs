use num_traits::Zero;
use std::{cell::RefCell, collections::VecDeque};

use anyhow::Result;
pub mod data;

type ValueType = u64;

pub struct Monkey {
    pub id: usize,
    pub items: VecDeque<Item>,
    pub operation: Box<dyn Fn(ValueType) -> ValueType>,
    pub test: Box<dyn Fn(ValueType) -> bool>,
    pub divisor: ValueType,
    pub true_monkey: usize,
    pub false_monkey: usize,
}
impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("true_monkey", &self.true_monkey)
            .field("false_monkey", &self.false_monkey)
            .finish()
    }
}
impl Monkey {
    pub fn new(lines: &mut impl Iterator<Item = String>) -> Result<Option<Monkey>> {
        let monkey_line = lines.next();
        let monkey_line = if let Some(ml) = monkey_line {
            ml
        } else {
            return Ok(None);
        };

        let mut id = monkey_line.trim().split_whitespace();

        if id
            .next()
            .ok_or_else(|| anyhow::anyhow!("COuld not get monkey"))?
            != "Monkey"
        {
            anyhow::bail!("Expected monkey");
        }
        // Semi colon at the end of id, remove that
        let id = id
            .next()
            .ok_or_else(|| anyhow::anyhow!("Could not get id"))?;
        let id = id
            .get(..id.len() - 1)
            .ok_or_else(|| anyhow::anyhow!("Could not trim id"))?;
        let id: usize = id.parse()?;

        let starting_items = lines
            .next()
            .ok_or_else(|| anyhow::anyhow!("Could not get starting items"))?;
        let starting_items = starting_items.trim().split(":");
        let starting_items = starting_items.last().ok_or_else(|| anyhow::anyhow!(""))?;
        let starting_items: Result<VecDeque<Item>> = starting_items
            .trim()
            .split(", ")
            .map(|i| i.try_into())
            .collect();
        let starting_items = starting_items?;

        let operation = parse_operation(
            &lines
                .next()
                .ok_or_else(|| anyhow::anyhow!("Could not get operation"))?,
        )?;

        let (test, divisor) = parse_test(
            &lines
                .next()
                .ok_or_else(|| anyhow::anyhow!("COuld not get test line"))?,
        )?;
        let iftrue = parse_if_true(
            &lines
                .next()
                .ok_or_else(|| anyhow::anyhow!("COuld not get if true line"))?,
        )?;
        let iffalse = parse_if_true(
            &lines
                .next()
                .ok_or_else(|| anyhow::anyhow!("COuld not get if false line"))?,
        )?;

        Ok(Some(Self {
            id,
            items: starting_items,
            operation,
            test,
            divisor,
            true_monkey: iftrue,
            false_monkey: iffalse,
        }))
    }
}

pub fn monkeys(data: &str) -> Result<Vec<Monkey>> {
    let mut data = data.lines().into_iter().map(|s| s.to_owned());

    let mut monkeys = Vec::new();
    while let Some(monkey) = Monkey::new(&mut data)? {
        monkeys.push(monkey);
        // eat blank line
        _ = data.next();
    }
    Ok(monkeys)
}

#[test]
fn test_monkey() -> Result<()> {
    let monkeys = monkeys(&data::sample_data()?)?;

    assert_eq!(monkeys.len(), 4);

    let m = &monkeys[2];
    // assert_eq!(
    //     m.items.iter().map(|i| i.value).collect::<Vec<_>>(),
    //     vec![79, 60, 97]
    // );

    // assert_eq!(m.items.front().unwrap().value, 79);

    assert_eq!(m.true_monkey, 1);
    assert_eq!(m.false_monkey, 3);

    Ok(())
}

pub fn parse_test(test: &str) -> Result<(Box<dyn Fn(ValueType) -> bool>, u64)> {
    let mut test = test.trim().split(" ");

    if test
        .next()
        .ok_or_else(|| anyhow::anyhow!("Could not get Test:"))?
        != "Test:"
    {
        anyhow::bail!("Expected Test:");
    }
    if test
        .next()
        .ok_or_else(|| anyhow::anyhow!("COuld not get divisible"))?
        != "divisible"
    {
        anyhow::bail!("Expected divisible");
    }
    if test
        .next()
        .ok_or_else(|| anyhow::anyhow!("Could not get by"))?
        != "by"
    {
        anyhow::bail!("Expected by");
    }
    let divisor: ValueType = test
        .next()
        .ok_or_else(|| anyhow::anyhow!("Could not get value"))?
        .parse()?;

    let op = move |v| {
        let divisor = divisor;
        v % divisor == Zero::zero()
    };

    Ok((Box::new(op), divisor))
}

pub fn parse_if_true(value: &str) -> Result<usize> {
    Ok(value
        .trim()
        .split_whitespace()
        .last()
        .ok_or_else(|| anyhow::anyhow!("Could not get value"))?
        .parse()?)
}

#[test]
fn test_parse_if_true() {
    assert_eq!(4, parse_if_true("  If true: trhow to monekey 4").unwrap());
}

#[test]
fn test_parsetest() -> Result<()> {
    let t = parse_test("  Test: divisible by 23")?.0;

    assert_eq!(true, t(23));
    assert_eq!(true, t(46));
    assert_eq!(false, t(47));

    Ok(())
}

pub fn parse_operation(op: &str) -> Result<Box<dyn Fn(ValueType) -> ValueType>> {
    let mut op = op.trim().split(" ");
    let operation = op
        .next()
        .ok_or_else(|| anyhow::anyhow!("Could not get operation"))?;
    let new = op
        .next()
        .ok_or_else(|| anyhow::anyhow!("Could not get new value"))?;
    let equals = op
        .next()
        .ok_or_else(|| anyhow::anyhow!("Could not get equals"))?;
    let old = op
        .next()
        .ok_or_else(|| anyhow::anyhow!("Could not get old value"))?;
    let real_op = op
        .next()
        .ok_or_else(|| anyhow::anyhow!("Could not get real operation"))?;
    let operand = op
        .next()
        .ok_or_else(|| anyhow::anyhow!("Could not get operand"))?;

    let operand: Option<u64> = match operand {
        "old" => None,
        _ => Some(operand.parse()?),
    };

    if operation != "Operation:" {
        anyhow::bail!("Operation not found");
    }
    if new != "new" {
        anyhow::bail!("New now found");
    }
    if equals != "=" {
        anyhow::bail!("Equals not found");
    }
    if old != "old" {
        anyhow::bail!("Old not found");
    }

    let op: Box<dyn Fn(ValueType) -> ValueType> = match real_op {
        "*" => Box::new(move |value| {
            if let Some(o) = operand {
                value * o
            } else {
                value.clone() * value
            }
        }),
        "+" => Box::new(move |value| {
            if let Some(o) = operand {
                value + o
            } else {
                value.clone() + value
            }
        }),
        _ => anyhow::bail!("Unknown operation {real_op}"),
    };

    Ok(op)
}

#[test]
fn test_op() -> Result<()> {
    let op = parse_operation("  Operation: new = old * 19")?;

    assert_eq!((19 * 3), op(3));

    Ok(())
}

#[derive(Debug)]
pub struct Item {
    pub value: ValueType,
    pub inspection_list: Vec<usize>,
}
impl TryFrom<&str> for Item {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            value: s
                .parse()
                .map_err(|e| anyhow::anyhow!("Couldn't parse item from {s}: {e}"))?,
            inspection_list: Vec::new(),
        })
    }
}

pub fn part1(data: &str) -> Result<String> {
    Ok(solver(data, 20, 3)?.to_string())
}

fn solver(data: &str, rounds: usize, worry: ValueType) -> Result<usize> {
    let monkeys = monkeys(data)?;
    // turn monkeys into refcells
    let monkeys: Vec<_> = monkeys.into_iter().map(|m| RefCell::new(m)).collect();

    // compute the multiple of all divisors.  This will keep our values in check (to a point)
    let value_modulus: ValueType = monkeys.iter().map(|m| m.borrow().divisor).product();

    for _round in 0..rounds {
        // We have to do index based array manipulation because of borrowing
        for i in 0..monkeys.len() {
            println!("Monkey {i}:");
            let mut m = monkeys
                .get(i)
                .ok_or_else(|| anyhow::anyhow!("deverr"))?
                .borrow_mut();

            while let Some(mut item) = m.items.pop_front() {
                println!(
                    "  Monkey inspects an item with a worry level of {}.",
                    item.value
                );
                item.inspection_list.push(i);
                let newvalue = (m.operation)(item.value);
                println!("  newvalue: {newvalue}");
                let newvalue = newvalue / worry.clone();
                let newvalue = newvalue % value_modulus;
                println!("  bored: {newvalue}");
                let mut newmonkey = if (m.test)(newvalue.clone()) {
                    println!("  true: give to {}", m.true_monkey);
                    monkeys
                        .get(m.true_monkey)
                        .ok_or_else(|| anyhow::anyhow!("COuld not get true monkey"))?
                } else {
                    println!("  false: give to {}", m.false_monkey);
                    monkeys
                        .get(m.false_monkey)
                        .ok_or_else(|| anyhow::anyhow!("COuld not get false monkey"))?
                }
                .borrow_mut();
                item.value = newvalue;
                newmonkey.items.push_back(item);
            }
        }
    }

    let mut counts = (0..monkeys.len())
        .into_iter()
        .map(|monkey_index| {
            monkeys
                .iter()
                .flat_map(|m| {
                    m.borrow()
                        .items
                        .iter()
                        .flat_map(|i| i.inspection_list.clone())
                        .collect::<Vec<_>>()
                })
                .filter(|i| *i == monkey_index)
                .count()
        })
        .enumerate()
        .collect::<Vec<_>>();

    counts.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{counts:?}");

    let products = counts
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("COuldn't get first count"))?
        .1
        * counts
            .get(1)
            .ok_or_else(|| anyhow::anyhow!("Could not get second count"))?
            .1;
    //println!("{monkeys:?}");

    Ok(products)
}

pub fn part2(data: &str) -> Result<String> {
    Ok(solver(data, 10000, 1)?.to_string())
}
