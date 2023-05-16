use std::{fs::read_to_string, str::Lines};
use anyhow::{Result, Context};

#[derive(Debug)]
struct Cpu {
    register: i64,
    register_values: Vec<i64>
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            register: 1,
            register_values: vec![1]
        }
    }

    fn execute_command(&mut self, command: &str) {
        if command == "noop" {
            self.register_values.push(self.register);
            return;
        }

        if command.starts_with("addx") {
            let change: i64 = command.split_whitespace().
                nth(1).expect("get addx arg").
                parse().expect("parse addx arg");
            self.register_values.push(self.register);
            self.register_values.push(self.register);
            self.register += change
        }
    }
}

fn main() -> Result<()>{
    // If first argument is "real", use the real input file
    // Otherwise, use the test input file
    let input_file = if std::env::args().nth(1).unwrap_or(String::default()).eq("real") {
        "real-input.txt"
    } else {
        "demo-input.txt"
    };
    println!("Using input file: {}", input_file);

    let input: String = read_to_string(input_file).context("failed to read the data file")?;
    let lines: Lines = input.lines();

    let mut cpu = Cpu::new();

    for command in lines {
        cpu.execute_command(command)
    }

    let mut strength = 0;
    for step in vec![20, 60, 100, 140, 180, 220] {
        strength += cpu.register_values[step] * step as i64
    }
    println!("Strength: {}", strength);
    Ok(())
}
