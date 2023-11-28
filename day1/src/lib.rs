use anyhow::Result;

#[derive(Clone)]
pub struct Elf {
    pub food: Vec<u32>,
}
impl Elf {
    pub fn total_cal(&self) -> u32 {
        self.food.iter().sum()
    }
}
pub struct CalorieElf<'a>(&'a Elf);
impl<'a> Ord for CalorieElf<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cal().cmp(&other.0.total_cal())
    }
}
impl<'a> Eq for CalorieElf<'a> {
}
impl<'a> PartialOrd for CalorieElf<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.total_cal().partial_cmp(&other.0.total_cal())
    }
}
impl<'a> PartialEq for CalorieElf<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cal().eq(&other.0.total_cal())
    }
}

pub struct Elves(pub Vec<Elf>);
impl Elves {
    pub fn max_calorie_elf(&self) -> Option<&Elf> {
        self.calorie_elves().max().map(|e| e.0)
    }
    fn calorie_elves(&self) -> impl Iterator<Item=CalorieElf> {
        self.0.iter().map(|e| CalorieElf(e))
    }
    pub fn sorted_by_calorie(&self) -> Vec<Elf> {
        let mut elves = self.0.clone();
        elves.sort_by(|a,b| a.total_cal().cmp(&b.total_cal()));
        elves
    }
}

pub fn read_food(file: &str) -> Result<Vec<Elf>> {
    let contents= std::fs::read_to_string(file)?;
    read_food_from_string(&contents)
}

fn read_food_from_string(contents: &str) -> Result<Vec<Elf>> {
    let lines = contents.lines();
    split_on_blank_line(lines).map(|food| Ok(Elf { food: food? })).collect()
}

fn split_on_blank_line<'a>(
    mut lines: impl Iterator<Item = &'a str> + 'a,
) -> impl Iterator<Item = Result<Vec<u32>>> + 'a {
    let block_of_lines = move || {
        let mut data = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let value = line.parse();
            match value {
                Ok(value) => data.push(value),
                Err(e) => return Some(Err(anyhow::anyhow!("Could not parse line: {line} {e}")))
            }
        }
        if data.is_empty() {
            None
        } else {
            Some(Ok(data))
        }
    };
    // turn block of lines fn into an iterator
    std::iter::from_fn(block_of_lines)
}

#[cfg(test)]
fn elves() -> Vec<Elf> {
    let sample = include_str!("../sample.txt");
    read_food_from_string(sample).unwrap()
}

#[test]
fn test_read_food() {
    let elves = elves();
    assert!(elves.len() == 5);
    assert_eq!(elves[0].total_cal(),6000);
    assert_eq!(elves[1].total_cal(),4000);
    assert_eq!(elves[2].total_cal(),11000);
    assert_eq!(elves[3].total_cal(),24000);
    assert_eq!(elves[4].total_cal(),10000);
}

#[test]
fn test_max_cal() {
    let elves = elves();
    let elves = Elves(elves);
    let max_elf = elves.max_calorie_elf().unwrap();
    assert_eq!(max_elf.total_cal(),24000);
}

#[test]
fn test_sorted() {
    let elves = Elves(elves());
    let mut sorted = elves.sorted_by_calorie();
    sorted.reverse();
    let cals_top_3 : u32 = sorted.into_iter().map(|e| e.total_cal()).take(3).sum();

    assert_eq!(cals_top_3,45000);
}