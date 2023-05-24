use std::{fs::read_to_string, str::Lines, collections::HashSet};

use anyhow::Context;

#[derive(Clone, PartialEq)]
enum Vol {
    Air,
    Rock,
    Water
}

fn main() {
    // If first argument is "real", use the real input file
    // Otherwise, use the test input file
    let input_type = std::env::args().nth(1).unwrap_or(String::default());
    let input_file = if input_type.eq("real") {
        "real-input.txt"
    } else {
        "demo-input.txt"
    };
    println!("Using input file: {}", input_file);

    let input: String = read_to_string(input_file).context("failed to read the data file").unwrap();
    let lines: Lines = input.lines();

    const VOL_LIMIT: usize = 25;
    let mut volume = vec![vec![vec![Vol::Air;VOL_LIMIT+1];VOL_LIMIT+1];VOL_LIMIT+1];

    for line in lines {
        let mut coords = line.split(",");
        let x: usize = coords.next().expect("reading x").parse().expect("parsing x");
        let y: usize = coords.next().expect("reading y").parse().expect("parsing y");
        let z: usize = coords.next().expect("reading z").parse().expect("parsing z");
        volume[x + 2][y + 2][z + 2] = Vol::Rock;
    }

    // Fill it with water starting with the origin
    let mut to_fill = vec![(1,1,1)];
    let mut external_rocks = HashSet::new();
    while let Some((x, y, z)) = to_fill.pop() {
        if x < 1 || y < 1 || z < 1 || x > VOL_LIMIT || y > VOL_LIMIT || z > VOL_LIMIT { continue }
        match volume[x][y][z] {
            Vol::Water => continue,
            Vol::Rock => { external_rocks.insert((x,y,z)); },
            Vol::Air => {
                volume[x][y][z] = Vol::Water;

                to_fill.push((x + 1, y, z));
                to_fill.push((x - 1, y, z));
                to_fill.push((x, y + 1, z));
                to_fill.push((x, y - 1, z));
                to_fill.push((x, y, z + 1));
                to_fill.push((x, y, z - 1));
            }
        }
    }

    let mut total_surface = 0;
    for (x,y,z) in external_rocks.into_iter() {
        if volume[x+1][y][z] == Vol::Water { total_surface += 1 }
        if volume[x-1][y][z] == Vol::Water { total_surface += 1 }
        if volume[x][y+1][z] == Vol::Water { total_surface += 1 }
        if volume[x][y-1][z] == Vol::Water { total_surface += 1 }
        if volume[x][y][z+1] == Vol::Water { total_surface += 1 }
        if volume[x][y][z-1] == Vol::Water { total_surface += 1 }
    }

    println!("Total surface: {}", total_surface);
}

// Real checks:
// 2069 - too low
// 2072 - correct
// 2173 - too high
