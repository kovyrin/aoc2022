use std::{fs::read_to_string, str::Lines, vec};
use anyhow::{Context, Result};

#[derive(Debug, Default, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn up(&self) -> Coord {
        Coord { x: self.x, y: self.y - 1 }
    }

    fn down(&self) -> Coord {
        Coord { x: self.x, y: self.y + 1 }
    }

    fn left(&self) -> Coord {
        Coord { x: self.x - 1, y: self.y }
    }

    fn right(&self) -> Coord {
        Coord { x: self.x + 1, y: self.y }
    }
}

#[derive(Debug)]
struct Step {
    coord: Coord,
    path_len: usize,
    src_height: char,
}

fn main() -> Result<()>{
    // If first argument is "real", use the real input file
    // Otherwise, use the test input file
    let input_file = if std::env::args().nth(1).unwrap_or(String::default()).eq("real") {
        "real-input.txt"
    } else {
        "demo-input.txt"
    };
    println!("Using input file: {input_file}");

    let input: String = read_to_string(input_file).context("failed to read the data file")?;
    let lines: Lines = input.lines();

    let mut map: Vec<Vec<char>> = Vec::default();

    let mut end = Coord::default();

    let mut cur_row: usize = 1;
    map.push(Vec::new()); // Add the top wall
    for line in lines {
        let mut map_row = vec!['~'; line.len() + 2];
        for (i, c) in line.chars().enumerate() {
            map_row[i + 1] = c;
        }

        if let Some(col) = map_row.iter().position(|c| *c == 'E') {
            end = Coord { x: col, y: cur_row };
            map_row[col] = 'z';
        }
        map.push(map_row);
        cur_row += 1;
    }

    let width = map[1].len();
    map[0] = vec!['~'; width];
    map.push(vec!['~'; width]);
    let height = map.len();

    println!("Map: {width}x{height}");
    println!("End: {:?}", end);
    for row in map.iter() {
        for col in row {
            print!("{col}");
        }
        println!();
    }

    // Create a map of visited places with distances from the start
    let mut path_len: Vec<Vec<usize>> = Vec::default();
    path_len.resize(height, vec![0;width]);

    let mut steps_to_check = Vec::default();
    for row in 0..height {
        for col in 0..width {
            if map[row][col] == 'a' {
                steps_to_check.push(Step { coord: Coord { x: col, y: row }, path_len: 0, src_height: 'a' })
            }
        }
    }

    let mut shortest_len = 1000000000;
    while let Some(step) = steps_to_check.pop() {
        // Do not take steps if the new path length is longer than whatever we have already found
        if step.path_len > shortest_len { continue }

        let step_height = map[step.coord.y][step.coord.x];
        let step_gain = elevation_gain(step.src_height, step_height);

        // Do not take steps with en elevation gain or drop that requires climbing
        if step_gain > 1 { continue }

        // This step would be a longer path to the given point than we have already found
        if path_len[step.coord.y][step.coord.x] > 0 && path_len[step.coord.y][step.coord.x] <= step.path_len  { continue }

        path_len[step.coord.y][step.coord.x] = step.path_len;
        if end.eq(&step.coord) && shortest_len > step.path_len {
            shortest_len = step.path_len;
        }

        steps_to_check.push(Step { coord: step.coord.up(), src_height: step_height, path_len: step.path_len + 1 });
        steps_to_check.push(Step { coord: step.coord.down(), src_height: step_height, path_len: step.path_len + 1 });
        steps_to_check.push(Step { coord: step.coord.left(), src_height: step_height, path_len: step.path_len + 1 });
        steps_to_check.push(Step { coord: step.coord.right(), src_height: step_height, path_len: step.path_len + 1 });
    }

    println!("Shortest path: {}", path_len[end.y][end.x]);
    Ok(())
}

fn elevation_gain(src: char, dst: char) -> i16 {
    dst as i16 - src as i16
}
