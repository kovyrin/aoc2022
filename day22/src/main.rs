use std::{fs::read_to_string, str::Lines};
use anyhow::Context;
use Direction::*;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn password_coefficient(&self) -> usize {
        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        }
    }

    fn turn_cw(&mut self) {
        *self = match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn turn_ccw(&mut self) {
        *self = match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn take_step(&self, dir: &Direction) -> Point {
        match dir {
            Up    => Point::new(self.x, self.y - 1),
            Right => Point::new(self.x + 1, self.y),
            Down  => Point::new(self.x, self.y + 1),
            Left  => Point::new(self.x - 1, self.y),
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
    let lines: Lines = input.lines();

    let mut map_lines = Vec::new();
    let mut finished_map_load = false;
    let mut algorithm = String::default();

    for line in lines {
        if line.is_empty() {
            finished_map_load = true;
            continue;
        }

        if !finished_map_load {
            map_lines.push(line);
        }

        if finished_map_load {
            algorithm = line.to_owned();
        }
    }

    let map_width = map_lines.iter().map(|line| line.len()).max().unwrap()+2;
    let map_height = map_lines.len()+2;

    let mut map = vec![vec![' '; map_width]; map_height];
    for (y, line) in map_lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map[y+1][x+1] = c;
        }
    }

    // Parse algorithm into a list of instructions
    // 10R5L5R10L4R5L5 means:
    // - Go forward 10 spaces
    // - Turn right
    // - Go forward 5 spaces
    // ...
    // - Turn left
    // - Go forward 5 spaces
    let mut instructions = Vec::new();
    let mut instruction = String::default();
    for c in algorithm.chars() {
        if c.is_digit(10) {
            instruction.push(c);
        } else {
            instructions.push(instruction);
            instructions.push(c.to_string());
            instruction = String::default();
        }
    }
    instructions.push(instruction);

    // Find the starting position
    let start_pos = Point {
        x: map[1].iter().position(|&c| c == '.').unwrap(),
        y: 1
    };

    // Execute the instructions
    let mut pos = start_pos;
    let mut dir = Right;
    for instruction in instructions {
        match instruction.as_str() {
            "R" => { dir.turn_cw() },
            "L" => { dir.turn_ccw() },
            steps => {
                let num_steps = steps.parse().unwrap();
                for _ in 0..num_steps {
                    if !flat_go_forward(&dir, &mut pos, &map) { break }
                }
            }
        }
    }

    print_results("part 1", &pos, &dir);
}

fn print_results(part: &str, pos: &Point, dir: &Direction) {
    println!("Final position for {}: {},{} with dir={:?}", part, pos.x, pos.y, dir);

    // Calculate the password:
    let dir_coeff = dir.password_coefficient();
    let password = 1000 * pos.y + 4 * pos.x + dir_coeff;
    println!("Password for {} = 1000 * {} + 4 * {} + {} = {}", part, pos.y, pos.x, dir_coeff, password);
}

fn flat_go_forward(dir: &Direction, pos: &mut Point, map: &Vec<Vec<char>>) -> bool {
    // Try to take a step
    let mut new_pos = pos.take_step(dir);

    // If we hit a void space (outside of the map), we need to wrap around to the other side of the map
    if map[new_pos.y][new_pos.x] == ' ' {
        new_pos = flat_wraparound_position(dir, pos, map);
    }

    // Step into the empty space
    if map[new_pos.y][new_pos.x] == '.' {
        *pos = new_pos;
        return true;
    }

    // If we hit a wall, stop
    if map[new_pos.y][new_pos.x] == '#' {
        return false
    }

    panic!("Unexpected character: {} at {},{}", map[new_pos.y][new_pos.x], new_pos.x, new_pos.y);
}

fn flat_wraparound_position(dir: &Direction, pos: &Point, map: &Vec<Vec<char>>) -> Point {
    let mut new_pos = pos.clone();

    // Go in the opposite direction until you hit a ' ', that is your new position
    let opposite_dir = dir.opposite();
    loop {
        let pos = new_pos.take_step(&opposite_dir);
        if map[pos.y][pos.x] == ' ' { return new_pos }
        new_pos = pos;
    }
}

// Final position for part 1: 3,164 with dir=Left
// Password for part 1 = 1000 * 164 + 4 * 3 + 2 = 164014
