use std::collections::HashSet;

use anyhow::Result;
use petgraph::graph::DiGraph;
pub mod data;

fn parse(data: &str) -> Result<(petgraph::Graph<i32, ()>, u32, u32, Vec<u32>)> {
    let mut grid = Vec::new();

    let mut source = None;
    let mut dest = None;

    for line in data.lines() {
        let mut row = Vec::new();
        for ch in line.chars() {
            let ch = match ch {
                'S' => {
                    source = Some((grid.len(), row.len()));
                    'a'
                }
                'E' => {
                    dest = Some((grid.len(), row.len()));
                    'z'
                }
                _ => ch,
            };
            let ascii: u8 = ch.try_into()?;
            let a: u8 = 'a'.try_into()?;
            let height = ascii - a;
            row.push(height);
        }
        grid.push(row);
    }

    // create all edges
    let mut edges = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid.first().map(|r| r.len()).unwrap_or(0) {
            let my_grid = (y, x);
            let my_grid_i32: (i32, i32) = (y.try_into()?, x.try_into()?);
            let my_height = grid[y][x];

            let mut test_dir = |off: (i32, i32)| -> Result<()> {
                let y: i32 = off.0 + my_grid_i32.0;
                let x: i32 = off.1 + my_grid_i32.1;
                // Intentionally fail this, if we cannot convert,
                let y: usize = y.try_into()?;
                let x: usize = x.try_into()?;

                let their_height = grid
                    .get(y)
                    .and_then(|row| row.get(x))
                    .ok_or_else(|| anyhow::anyhow!("Grid not accessible"))?;
                if *their_height <= my_height + 1 {
                    edges.push((my_grid, (y, x)));
                }
                Ok(())
            };

            // We explicitly ignore the result because the err result indicates
            // an out of bounds attempt;
            _ = test_dir((-1, 0));
            _ = test_dir((1, 0));
            _ = test_dir((0, -1));
            _ = test_dir((0, 1));
        }
    }

    //println!("{grid:?}");
    //println!("{edges:?}");

    let row_len = grid.first().map(|row| row.len()).unwrap_or(0);
    let to_unwound = |(y, x): (usize, usize)| -> u32 { (y * row_len + x).try_into().unwrap() };

    // Convert edges to a linear space
    let edges = edges
        .into_iter()
        .map(|(from, to)| (to_unwound(from), to_unwound(to)))
        .collect::<Vec<_>>();

    let graph = petgraph::Graph::<i32, ()>::from_edges(&edges);

    // find all the start points
    let startpoints = grid
        .into_iter()
        .flat_map(|row| row)
        .enumerate()
        .filter(|(_, height)| *height == 0)
        .map(|(i, _)| i as u32);

    Ok((
        graph,
        to_unwound(source.unwrap()),
        to_unwound(dest.unwrap()),
        startpoints.collect(),
    ))
}

// use petgraph::algo::simple_paths::all_simple_paths;
// let all_paths = all_simple_paths::<Vec<_>, _>(
//     &graph,
//     to_unwound(source.unwrap()).into(),
//     to_unwound(dest.unwrap()).into(),
//     0,
//     None,
// );

//    let shortest = all_paths.map(|p| p.len()).min().ok_or_else(|| anyhow::anyhow!("No shortest path"))? - 1;
// let mut path = Default::default();
// let mut paths = Default::default();
// let mut visited = Default::default();
// let mut shortest = None;
// find_paths(
//     &graph,
//     to_unwound(source.unwrap()),
//     to_unwound(dest.unwrap()),
//     &mut visited,
//     &mut path,
//     &mut paths,
//     &mut shortest
// );

// let shortest = paths
//     .into_iter()
//     .map(|p| p.len())
//     .min()
//     .ok_or_else(|| anyhow::anyhow!("no shortest path"))?
//     - 1;
// println!("{shortest}");

fn find_paths(
    graph: &DiGraph<i32, ()>,
    start: u32,
    end: u32,
    visited: &mut HashSet<u32>,
    path: &mut Vec<u32>,
    all_paths: &mut Vec<Vec<u32>>,
    shortest: &mut Option<usize>,
) {
    if let Some(shortest) = shortest {
        if path.len() + 1 >= *shortest {
            return;
        }
    }

    println!("{:?}", path);

    visited.insert(start);
    path.push(start);

    if start == end {
        println!("Found path len {}", path.len());
        all_paths.push(path.clone());
        shortest.replace(path.len());
    } else {
        for neighbor in graph.neighbors(start.into()) {
            if !visited.contains(&(neighbor.index() as u32)) {
                find_paths(
                    graph,
                    neighbor.index() as u32,
                    end,
                    visited,
                    path,
                    all_paths,
                    shortest,
                );
            }
        }
    }

    path.pop();
    visited.remove(&start);
}

#[allow(unused)]
pub fn part1(data: &str) -> Result<String> {
    let (graph, source, dest, _) = parse(data)?;

    let dest = dest.into();

    let path = petgraph::algo::astar(
        &graph,
        source.into(), // start
        |n| n == dest, // is_goal
        |e| 1,         // edge_cost
        |_| 0,         // estimate_cost
    )
    .ok_or_else(|| anyhow::anyhow!("no short path"))?;

    //println!("{path:?}");

    Ok(path.0.to_string())
}

#[allow(unused)]
pub fn part2(data: &str) -> Result<String> {
    let (graph, source, dest, startpoints) = parse(data)?;

    let dest = dest.into();

    let shortest = startpoints.into_iter().filter_map(|source| {
        let path = petgraph::algo::astar(
            &graph,
            source.into(), // start
            |n| n == dest, // is_goal
            |e| 1,         // edge_cost
            |_| 0,         // estimate_cost
        )?;
//        .ok_or_else(|| anyhow::anyhow!("no short path"))?;
        Some(path.0)
    }).min();

    Ok(shortest.ok_or_else(|| anyhow::anyhow!("no shortest"))?.to_string())
}
