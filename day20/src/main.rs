use std::{fs::read_to_string, str::Lines};
use anyhow::Context;

#[derive(Debug, Clone)]
struct Number {
    value: i32,
    shifted: bool,
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

    let mut numbers: Vec<Number> = lines.map(|l| Number {
        value: l.parse::<i32>().unwrap(),
        shifted: false,
    }).collect();

    // println!("Initial state:");
    // println!("{:?}\n", numbers.iter().map(|n| n.value).collect::<Vec<i32>>());

    let modulo = numbers.len() as i32 - 1;
    while let Some((old_pos, num)) = numbers.iter().enumerate().find(|(_, num)| !num.shifted) {
        let value = num.value;
        numbers[old_pos].shifted = true;

        if value == 0 { continue }

        // Find the new position for the number shifting it by its value, wrapping around if needed
        let new_pos = (old_pos as i32 + value).rem_euclid(modulo) as usize;

        // If the new position is the same as the old position, no need to do anything
        if new_pos == old_pos { continue }

        // println!("Moving {} from {} to {}", value, old_pos, new_pos);
        if old_pos < new_pos {
            numbers[old_pos..=new_pos].rotate_left(1);
        } else {
            numbers[new_pos..=old_pos].rotate_right(1);
        }

        // println!("{:?}\n", numbers.iter().map(|n| n.value).collect::<Vec<i32>>());
    }

    // Coordinates x,y,z are found at positions 1000, 2000, 3000 after the 0 in the list
    let zero_pos = numbers.iter().enumerate().find(|(_, num)| num.value == 0).unwrap().0;
    let x_pos = (zero_pos + 1000) % numbers.len();
    let y_pos = (zero_pos + 2000) % numbers.len();
    let z_pos = (zero_pos + 3000) % numbers.len();

    println!("x: {}", numbers[x_pos].value);
    println!("y: {}", numbers[y_pos].value);
    println!("z: {}", numbers[z_pos].value);

    println!("Sum of coordinates: {}", numbers[x_pos].value + numbers[y_pos].value + numbers[z_pos].value);
}


// Real checks:
// - 3063 is too low
// - 8302 is good
