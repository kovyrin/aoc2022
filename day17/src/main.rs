use std::{str::Lines, fs::read_to_string};
use anyhow::Context;

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

impl Jet {
    fn from_char(c: char) -> Self {
        match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            c => panic!("Unexpected character: '{c}'")
        }
    }
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
    let mut lines: Lines = input.lines();
    let jets: Vec<Jet> = lines.next().expect("reading jets").chars().map(|c| Jet::from_char(c)).collect();

    for jet in jets.iter() {
        println!("Command: {:?}", jet);
    }
}
