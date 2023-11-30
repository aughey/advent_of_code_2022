#![allow(unused)]
use std::{cell::RefCell, default, rc::Rc};

use anyhow::Result;

#[derive(Default, Debug)]
struct Builder {
    stack: Vec<Rc<RefCell<Dir>>>,
    root: Rc<RefCell<Dir>>,
}

impl Builder {
    fn cwd(&mut self) -> Rc<RefCell<Dir>> {
        self.stack
            .last()
            .map(|v| v.clone())
            .unwrap_or(self.root.clone())
    }
    pub fn act<'a>(
        &mut self,
        command: &Command,
        next_lines: &'a mut std::iter::Peekable<impl Iterator<Item = String>>,
    ) -> Result<()> {
        match command {
            Command::CD(dir) => match dir.as_str() {
                ".." => {
                    self.stack
                        .pop()
                        .ok_or_else(|| anyhow::anyhow!("Attempted to go up past root"));
                }
                "/" => while self.stack.pop().is_some() {},
                _ => {
                    // find dir in our current dir
                    let dir = self.cwd().borrow_mut().find_dir(dir.as_str())?;
                    self.stack.push(dir);
                }
            },
            Command::LS => {
                loop {
                    let next = next_lines.peek();
                    match next {
                        Some(line) => {
                            if line.starts_with("$") {
                                break;
                            }
                            // consume
                            let line =
                                next_lines.next().ok_or_else(|| anyhow::anyhow!("DevErr"))?;
                            let entry: Entry = line.as_str().try_into()?;
                            self.cwd().borrow_mut().entries.push(entry);
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

enum Command {
    CD(String),
    LS,
}
impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut value = line.split(" ").into_iter();
        let dollar = value
            .next()
            .ok_or_else(|| anyhow::anyhow!("COuldn't get dollar"))?;
        if dollar != "$" {
            anyhow::bail!("expected dollar got {dollar} for line {line}");
        }
        let command = value.next().ok_or_else(|| anyhow::anyhow!("No command"))?;
        let c = match command {
            "cd" => {
                let dir = value
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("No directory for cd command"))?;
                Command::CD(dir.to_string())
            }
            "ls" => Command::LS,
            _ => {
                anyhow::bail!("invalid command {command}");
            }
        };
        Ok(c)
    }
}

#[test]
fn test_command() -> Result<()> {
    let c: Command = "$ ls".try_into()?;
    assert!(matches!(c, Command::LS));
    let c: Command = "$ cd foo".try_into()?;
    assert!(matches!(c, Command::CD(_)));
    Ok(())
}

#[derive(Debug)]
enum Entry {
    File(File),
    Dir(Rc<RefCell<Dir>>),
}
impl TryFrom<&str> for Entry {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut value = value.split(" ");
        let file_or_dir = value
            .next()
            .ok_or_else(|| anyhow::anyhow!("Couldn't get next entry"))?;
        Ok(match file_or_dir {
            "dir" => {
                let name = value
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("Couldn't get name from dir entry"))?;
                Self::Dir(Rc::new(RefCell::new(Dir {
                    name: name.to_string(),
                    ..Default::default()
                })))
            }
            _ => {
                // expect file
                let size = file_or_dir.parse()?;
                let name = value
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("Couldn't get name for file"))?
                    .to_string();
                Self::File(File { size, name })
            }
        })
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

#[derive(Default, Debug)]
struct Dir {
    name: String,
    entries: Vec<Entry>,
}
impl Dir {
    fn at_most<'a>(&'a self, max: u64, accum: &'a mut Vec<Rc<RefCell<Dir>>>) {
        for e in self.entries.iter() {
            if let Entry::Dir(d) = e {
                if d.borrow().size() <= max {
                    accum.push(d.clone());
                }
                d.borrow().at_most(max, accum);
            }
        }
    }
    fn at_least<'a>(&'a self, max: u64, accum: &'a mut Vec<Rc<RefCell<Dir>>>) {
        for e in self.entries.iter() {
            if let Entry::Dir(d) = e {
                if d.borrow().size() >= max {
                    accum.push(d.clone());
                }
                d.borrow().at_least(max, accum);
            }
        }
    }
    fn size(&self) -> u64 {
        let mut accum = 0u64;
        for e in self.entries.iter() {
            match e {
                Entry::File(f) => accum += f.size,
                Entry::Dir(d) => accum += d.borrow().size(),
            }
        }
        accum
    }
    fn find_dir(&self, name: &str) -> Result<Rc<RefCell<Dir>>> {
        for entry in self.entries.iter() {
            if let (Entry::Dir(d)) = entry {
                if d.borrow().name == name {
                    return Ok(d.clone());
                }
            }
        }
        anyhow::bail!("Could not find directory name {name}");
    }
}

#[allow(unused)]
fn part1(data: &str) -> Result<String> {
    let data = data.lines().map(|l| l.to_owned()).into_iter();
    let mut data = data.peekable();

    let mut builder = Builder::default();
    while let Some(line) = data.next() {
        let command: Command = line.as_str().try_into()?;
        builder.act(&command, &mut data);
    }
    let root = builder.root.borrow();

    let mut atmost = Vec::new();
    root.at_most(100000, &mut atmost);
    let sum: u64 = atmost.into_iter().map(|d| d.borrow().size()).sum();
    Ok(sum.to_string())
}

#[allow(unused)]
fn part2(data: &str) -> Result<String> {
    let data = data.lines().map(|l| l.to_owned()).into_iter();
    let mut data = data.peekable();

    let mut builder = Builder::default();
    while let Some(line) = data.next() {
        let command: Command = line.as_str().try_into()?;
        builder.act(&command, &mut data);
    }
    let root = builder.root.borrow();

    const TOTAL: u64 = 70000000;
    const NEED: u64 = 30000000;
    
    let currently_used = root.size();
    let unused = TOTAL - currently_used;
    let need_to_free = NEED - unused;

    let mut possible = Vec::new();
    root.at_least(need_to_free, &mut possible);
    let mut possible : Vec<u64> = possible.into_iter().map(|d| d.borrow().size()).collect();
    // find the samllest
    possible.sort();

    let ans = possible.first().ok_or_else(||anyhow::anyhow!("no possible candidates"))?;

    Ok(ans.to_string())
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
    let expected = "95437".to_string();
    let got = part1(sample_data()?.as_str())?;

    assert_eq!(expected, got);
    Ok(())
}

#[test]
#[allow(unused)]
fn test_part2() -> Result<()> {
    let expected = "24933642".to_string();
    let got = part2(sample_data()?.as_str())?;

    assert_eq!(expected, got);
    Ok(())
}
