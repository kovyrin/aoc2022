use std::{fs::read_to_string, str::Lines};

use anyhow::Context;

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

    let mut volume = vec![vec![vec![false;100];100];100];

    for line in lines {
        let mut coords = line.split(",");
        let x: usize = coords.next().expect("reading x").parse().expect("parsing x");
        let y: usize = coords.next().expect("reading y").parse().expect("parsing y");
        let z: usize = coords.next().expect("reading z").parse().expect("parsing z");
        volume[x + 1][y + 1][z + 1] = true;
    }

    let mut total_surface = 0;
    for x in 1..100 {
        for y in 1..100 {
            for z in 1..100 {
                if !volume[x][y][z] { continue }

                let mut surface = 6;
                if volume[x+1][y][z] { surface -= 1 }
                if volume[x-1][y][z] { surface -= 1 }
                if volume[x][y+1][z] { surface -= 1 }
                if volume[x][y-1][z] { surface -= 1 }
                if volume[x][y][z+1] { surface -= 1 }
                if volume[x][y][z-1] { surface -= 1 }
                total_surface += surface;
            }
        }
    }

    println!("Total surface: {}", total_surface);
}
