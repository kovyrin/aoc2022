use std::{fs::read_to_string, str::Lines};
use anyhow::{Result, Context};

#[derive(Debug)]
struct Computer {
    sprite_center: i64,
    pixel_pos: usize,
    crt: Vec<bool>
}

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

impl Computer {
    fn new() -> Self {
        let mut c = Computer {
            sprite_center: 1,
            pixel_pos: 0,
            crt: Vec::with_capacity(WIDTH * HEIGHT)
        };
        c.crt.resize(WIDTH * HEIGHT, false);
        c
    }

    fn execute_command(&mut self, command: &str) {
        if command == "noop" {
            self.color_pixel();
            return;
        }

        if command.starts_with("addx") {
            let change: i64 = command.split_whitespace()
                .nth(1).expect("get addx arg")
                .parse().expect("parse addx arg");
            self.color_pixel();
            self.color_pixel();
            self.sprite_center += change
        }
    }

    fn pixel_char(&self, value: bool) -> char {
        if value { '#' } else { ' ' }
    }

    fn color_pixel(&mut self) {
        let sprite = self.sprite_center-1..=self.sprite_center+1;
        let pixel_pos = (self.pixel_pos % WIDTH) as i64;
        let pixel = sprite.contains(&pixel_pos);
        self.crt[self.pixel_pos] = pixel;
        self.pixel_pos += 1;
    }

    fn print_screen(&self) {
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                let pos = row*WIDTH + col;
                print!("{}", self.pixel_char(self.crt[pos]))
            }
            println!()
        }
        println!();
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

    let mut computer = Computer::new();

    for command in lines {
        computer.execute_command(command)
    }

    computer.print_screen();

    Ok(())
}
