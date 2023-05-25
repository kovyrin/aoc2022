use std::{fs::read_to_string, str::Lines};
use anyhow::Context;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Number {
    value: i64,
    org_pos: usize,
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

    let numbers: Vec<Number> = lines.enumerate().map(|(pos, line)|
        Number { value: line.parse().unwrap(), org_pos: pos }
    ).collect();

    let mut results = numbers.clone();
    let modulo = numbers.len() as i64 - 1;
    for number in numbers.iter() {
        let old_pos = results.iter().position(|num| num == number ).unwrap();

        // Find the new position for the number shifting it by its value, wrapping around if needed
        let new_pos = (old_pos as i64 + number.value).rem_euclid(modulo) as usize;

        if old_pos < new_pos {
            results[old_pos..=new_pos].rotate_left(1);
        } else {
            results[new_pos..=old_pos].rotate_right(1);
        }
    }

    // Coordinates x,y,z are found at positions 1000, 2000, 3000 after the 0 in the list
    let zero_pos = results.iter().enumerate().find(|(_, num)| num.value == 0).unwrap().0;
    let x_pos = (zero_pos + 1000) % results.len();
    let y_pos = (zero_pos + 2000) % results.len();
    let z_pos = (zero_pos + 3000) % results.len();

    println!("x: {}", results[x_pos].value);
    println!("y: {}", results[y_pos].value);
    println!("z: {}", results[z_pos].value);

    println!("Sum of coordinates: {}", results[x_pos].value + results[y_pos].value + results[z_pos].value);
}


// Real checks:
// - 3063 is too low
// - 8302 is good
