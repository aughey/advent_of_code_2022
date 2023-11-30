use anyhow::Result;

#[derive(Default, Debug)]
pub struct Command {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}
impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.split(' ');
        let mut get = move || {
            value
                .next()
                .ok_or_else(|| anyhow::anyhow!("Couldn't get next in command"))
        };
        let _move = get()?;
        let count = get()?.parse()?;
        let _from = get()?;
        let from: usize = get()?.parse()?;
        let _to = get()?;
        let to: usize = get()?.parse()?;
        Ok(Self {
            count,
            from: from - 1,
            to: to - 1,
        })
    }
}

#[test]
fn test_command() -> Result<()> {
    let c: Command = "move 1 from 2 to 3".try_into()?;
    assert_eq!(c.count, 1);
    assert_eq!(c.from, 1);
    assert_eq!(c.to, 2);
    Ok(())
}

#[derive(Clone, Default, Debug)]
pub struct Stack {
    pub items: Vec<String>,
}
impl Stack {
    pub fn top(&self) -> String {
        self.items
            .last()
            .map(|v| v.as_str())
            .unwrap_or(" ")
            .to_owned()
    }
}

#[derive(Clone, Debug)]
pub struct Stacks {
    pub stacks: Vec<Stack>,
}

impl Stacks {
    pub fn execute(&mut self, command: &Command) -> Result<()> {
        let stacks = &mut self.stacks;
        for _ in 0..command.count {
            let from = stacks
                .get_mut(command.from)
                .ok_or_else(|| anyhow::anyhow!("Could not get from stack {}", command.from))?;

            let value = from
                .items
                .pop()
                .ok_or_else(|| anyhow::anyhow!("Could not remove item from stack"))?;

            let to = stacks
                .get_mut(command.to)
                .ok_or_else(|| anyhow::anyhow!("Could not get from stack {}", command.to))?;

            to.items.push(value);
        }
        Ok(())
    }

    pub fn execute_9001(&mut self, command: &Command) -> Result<()> {
        let stacks = &mut self.stacks;
        let mut values = Vec::new();
        for _ in 0..command.count {
            let from = stacks
                .get_mut(command.from)
                .ok_or_else(|| anyhow::anyhow!("Could not get from stack {}", command.from))?;

            let value = from
                .items
                .pop()
                .ok_or_else(|| anyhow::anyhow!("Could not remove item from stack"))?;

            values.push(value);
        }
        values.reverse();

        let to = stacks
            .get_mut(command.to)
            .ok_or_else(|| anyhow::anyhow!("Could not get from stack {}", command.to))?;
        
        for value in values {
            to.items.push(value);
        }
        Ok(())
    }

    pub fn across(&self) -> String {
        self.stacks.iter().map(|s| s.top()).collect()
    }
}

impl TryFrom<Vec<&str>> for Stacks {
    type Error = anyhow::Error;

    fn try_from(mut stacks: Vec<&str>) -> Result<Self, Self::Error> {
        // Read it from bottom up
        stacks.reverse();
        let mut stacks = stacks.into_iter();

        // First line is the stack numbers
        let stack_count = stacks
            .next()
            .ok_or_else(|| anyhow::anyhow!("Couldn't get stack numbers"))?
            .trim()
            .split_whitespace()
            .count();

        // allocate the stacks
        let mut ss: Vec<Stack> = vec![Default::default(); stack_count];

        for line in stacks {
            // Each value occupies 3 spaces
            let mut line = line;

            let mut index = 0usize;
            while line.len() >= 3 {
                let (value, _) = line.split_at(3);
                assert_eq!(value.len(), 3);
                let s = ss
                    .get_mut(index)
                    .ok_or_else(|| anyhow::anyhow!("Could not get stack at index {index}"))?;
                let item = value
                    .chars()
                    .nth(1)
                    .ok_or_else(|| anyhow::anyhow!("DevERror"))?
                    .to_owned()
                    .to_string();
                if item != " " {
                    s.items.push(item);
                }
                line = line.split_at(4.min(line.len())).1;
                index += 1;
            }
        }

        Ok(Stacks { stacks: ss })
    }
}

#[test]
fn test_split() {
    let input = " 1   2   3 ";
    let input = input
        .trim()
        .split_whitespace()
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(input.len(), 3);
}

pub fn split_stack_commands(value: &str) -> Result<(Vec<&str>, Vec<&str>)> {
    let mut lines = value.lines().collect::<Vec<_>>();
    // find the index of the blank line
    let blank_index = lines
        .iter()
        .enumerate()
        .find(|l| l.1.is_empty())
        .ok_or_else(|| anyhow::anyhow!("No blank index"))?
        .0;
    lines.remove(blank_index);
    let s = lines.split_at(blank_index);
    Ok((s.0.to_vec(), s.1.to_vec()))
}

#[test]
fn test_split_stack() -> Result<()> {
    let data = sample_data()?;
    let res = split_stack_commands(data.as_str())?;

    assert_eq!(res.0.len(), 4);
    assert_eq!(res.1.len(), 4);
    assert_eq!(res.1[0], "move 1 from 2 to 1");

    Ok(())
}

#[test]
fn test_stack_reader() -> Result<()> {
    let data = sample_data()?;
    let res = split_stack_commands(data.as_str())?;
    let stacks: Stacks = res.0.try_into()?;

    println!("{:?}", stacks);

    assert_eq!(stacks.stacks.len(), 3);
    assert_eq!(stacks.stacks[0].items.len(), 2);

    Ok(())
}

#[allow(unused)]
fn part1(data: &str) -> Result<String> {
    let data = split_stack_commands(data)?;
    let mut stacks: Stacks = data.0.try_into()?;

    for c in data.1 {
        let c: Command = c.try_into()?;
        stacks.execute(&c);
    }

    println!("{stacks:?}");

    Ok(stacks.across())
}

#[allow(unused)]
fn part2(data: &str) -> Result<String> {
    let data = split_stack_commands(data)?;
    let mut stacks: Stacks = data.0.try_into()?;

    for c in data.1 {
        let c: Command = c.try_into()?;
        stacks.execute_9001(&c);
    }

    Ok(stacks.across())
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
    let expected = "CMZ".to_string();
    let got = part1(sample_data()?.as_str())?;

    assert_eq!(expected, got);
    Ok(())
}

#[test]
#[allow(unused)]
fn test_part2() -> Result<()> {
    let expected = "MCD".to_string();
    let got = part2(sample_data()?.as_str())?;

    assert_eq!(expected, got);
    Ok(())
}
