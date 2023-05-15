use std::{fs::read_to_string, str::Lines};
use anyhow::Context;
use colored::Colorize;

type Map = Vec<Vec<u8>>;

fn main() {
    // If first argument is "real", use the real input file
    // Otherwise, use the test input file
    let input_file = if std::env::args().nth(1).unwrap_or(String::default()).eq("real") {
        "real-input.txt"
    } else {
        "demo-input.txt"
    };
    println!("Using input file: {}", input_file);

    let input: String = read_to_string(input_file).context("failed to read the data file").unwrap();
    let lines: Lines = input.lines();

    let mut height_map: Map = Vec::default();
    let mut visibility_map: Map = Vec::default();

    for line in lines {
        let height_row = line.chars().map(|c| c as u8 - '0' as u8).collect();
        height_map.push(height_row);
        visibility_map.push(vec![0; line.len()]);
    }

    let height = height_map.len();
    let width = height_map[0].len();

    println!("Map ({}x{}):", width, height);
    for row in &height_map {
        for col in row { print!("{}", col) }
        println!("");
    }

    // Check rows
    for row in 0..height {
        visibility_map[row][0] = 1;
        visibility_map[row][width-1] = 1;

        // Check from the left
        let mut obstacle = height_map[row][0];
        for col in 1..width {
            if height_map[row][col] > obstacle {
                visibility_map[row][col] = 1;
                obstacle = height_map[row][col];
            }
        }

        // Check from the right
        obstacle = height_map[row][width-1];
        for col in (1..width-1).rev() {
            if height_map[row][col] > obstacle {
                visibility_map[row][col] = 1;
                obstacle = height_map[row][col];
            }
        }
    }

    // Check cols
    for col in 0..width {
        visibility_map[0][col] = 1;
        visibility_map[height-1][col] = 1;

        // Check from the left
        let mut obstacle = height_map[0][col];
        for row in 1..height {
            if height_map[row][col] > obstacle {
                visibility_map[row][col] = 1;
                obstacle = height_map[row][col];
            }
        }

        // Check from the right
        obstacle = height_map[width-1][col];
        for row in (1..height-1).rev() {
            if height_map[row][col] > obstacle {
                visibility_map[row][col] = 1;
                obstacle = height_map[row][col];
            }
        }
    }

    println!("--------------------------------------------------");
    println!("Visibility Map ({}x{}):", width, height);
    let mut total_visible = 0;
    for row in 0..width {
        print!("{}\t", row);
        for col in 0..height {
            let visible = visibility_map[row][col];
            let h = height_map[row][col].to_string();

            if visible == 1 {
                total_visible += 1;
                print!("{}", h.green());
            } else {
                print!("{}", h.white());
            }
        }
        println!("");
    }

    println!("Total visible: {}", total_visible)
}
