use std::num::ParseIntError;

use anyhow::Result;

#[derive(Debug)]
struct Grid {
    trees: Vec<Vec<u64>>,
}
impl Grid {
    fn all_points(&self) -> impl Iterator<Item = (usize, usize)> {
        let mut points = Vec::new();
        for y in 0..self.trees.len() {
            for x in 0..self.trees.len() {
                points.push((x, y))
            }
        }
        points.into_iter()
    }
    fn width(&self) -> usize {
        self.trees.first().map(|row| row.len()).unwrap_or(0)
    }
    fn height(&self) -> usize {
        self.trees.len()
    }
    fn is_visible(&self, point: (usize, usize)) -> Result<bool> {
        let myheight = *self
            .trees
            .get(point.1)
            .ok_or_else(|| anyhow::anyhow!("Could not get y point"))?
            .get(point.0)
            .ok_or_else(|| anyhow::anyhow!("Could not get x point"))?;
        let left = (0..point.0).map(|x| self.trees[point.1][x]);
        let right = (point.0 + 1..self.width()).map(|x| self.trees[point.1][x]);
        let up = (0..point.1).map(|y| self.trees[y][point.0]);
        let down = (point.1 + 1..self.height()).map(|y| self.trees[y][point.0]);

        if is_visible(left, myheight)
            || is_visible(right, myheight)
            || is_visible(up, myheight)
            || is_visible(down, myheight)
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn house_score(&self, point: (usize, usize)) -> Result<usize> {
        let myheight = *self
            .trees
            .get(point.1)
            .ok_or_else(|| anyhow::anyhow!("Could not get y point"))?
            .get(point.0)
            .ok_or_else(|| anyhow::anyhow!("Could not get x point"))?;
        let left = (0..point.0).map(|x| self.trees[point.1][x]).rev();
        let right = (point.0 + 1..self.width()).map(|x| self.trees[point.1][x]);
        let up = (0..point.1).map(|y| self.trees[y][point.0]).rev();
        let down = (point.1 + 1..self.height()).map(|y| self.trees[y][point.0]);

        Ok(visible_trees(left, myheight)
            * visible_trees(right, myheight)
            * visible_trees(up, myheight)
            * visible_trees(down, myheight))
    }
}

fn is_visible(mut it: impl Iterator<Item = u64>, myheight: u64) -> bool {
    !it.any(|v| v >= myheight)
}

fn visible_trees(it: impl Iterator<Item = u64>, myheight: u64) -> usize {
    let mut accum = 0usize;
    for v in it {
        accum += 1;
        if v >= myheight {
            break;
        }
    }
    accum
}

impl TryFrom<&str> for Grid {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut trees = Vec::new();
        for line in value.lines() {
            let row: Result<Vec<u64>, ParseIntError> =
                line.chars().map(|v| v.to_string().parse()).collect();
            let row = row?;
            trees.push(row);
        }
        Ok(Self { trees })
    }
}

#[allow(unused)]
fn part1(data: &str) -> Result<String> {
    let trees: Grid = data.try_into()?;

    println!("{trees:?}");

    let mut accum = 0usize;
    for point in trees.all_points() {
        if trees.is_visible(point)? {
            accum += 1
        }
    }

    Ok(accum.to_string())
}

#[allow(unused)]
fn part2(data: &str) -> Result<String> {
    let trees: Grid = data.try_into()?;

    println!("{:?}",trees.all_points().map(|p| trees.house_score(p)).collect::<Vec<_>>());

    _ = trees.house_score((2,1));

    let mut max_score = None;
    for point in trees.all_points() {
        let score = trees.house_score(point)?;
        if score > max_score.unwrap_or(0) {
            max_score = Some(score)
        }
    }
    Ok(max_score.unwrap_or(0).to_string())
}

#[cfg(test)]
fn sample_data() -> Result<String> {
    Ok(include_str!("../sample.txt").to_string())
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
    let expected = "21".to_string();
    let got = part1(sample_data()?.as_str())?;

    assert_eq!(expected, got);
    Ok(())
}

#[test]
#[allow(unused)]
fn test_part2() -> Result<()> {
    let expected = "8".to_string();
    let got = part2(sample_data()?.as_str())?;

    assert_eq!(expected, got);
    Ok(())
}
