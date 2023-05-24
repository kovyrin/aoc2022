use std::{fs::read_to_string, str::Lines};

use anyhow::Context;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
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

    let points: Vec<Point> = lines.map( |line| {
        let mut coords = line.split(",");
        let x: i32 = coords.next().expect("reading x").parse().expect("parsing x");
        let y: i32 = coords.next().expect("reading y").parse().expect("parsing y");
        let z: i32 = coords.next().expect("reading z").parse().expect("parsing z");
        Point { x, y, z }
    }).collect();

    for point in points.iter() {
        println!("{:?}", point);
    }
}
