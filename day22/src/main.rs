use std::{fs::read_to_string, str::Lines};
use anyhow::Context;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
    let mut pos = Point {
        x: map[1].iter().position(|&c| c == '.').unwrap(),
        y: 1
    };

    let mut dir = Direction::Right;

    // Execute the instructions
    for instruction in instructions {
        match instruction.as_str() {
            "R" => { dir = turn_cw(dir) },
            "L" => { dir = turn_ccw(dir) },
            steps => {
                let num_steps = steps.parse::<usize>().unwrap();
                for _ in 0..num_steps {
                    if !go_forward(&dir, &mut pos, &map) {
                        break;
                    }
                }
            }
        }
        println!("Instruction: {}, new position: {},{} with dir={:?}", instruction, pos.x, pos.y, dir);
    }

    println!("Final position: {},{} with dir={:?}", pos.x, pos.y, dir);

    // Calculate the password:
    let dir_coeff = match dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };
    let password = 1000 * pos.y + 4 * pos.x + dir_coeff;
    println!("Password = 1000 * {} + 4 * {} + {} = {}", pos.y, pos.x, dir_coeff, password);
}

fn turn_cw(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn turn_ccw(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Left,
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
    }
}

fn go_forward(dir: &Direction, pos: &mut Point, map: &Vec<Vec<char>>) -> bool {
    // Try to take a step
    let mut new_pos = calc_new_position(dir, pos);

    // If we hit a void space (outside of the map), we need to wrap around to the other side of the map
    if map[new_pos.y][new_pos.x] == ' ' {
        new_pos = wraparound_calc_new_position(dir, pos, map);
    }

    // Step into the empty space
    if map[new_pos.y][new_pos.x] == '.' {
        *pos = new_pos;
        return true;
    }

    // If we hit a wall, stop
    if map[new_pos.y][new_pos.x] == '#' { return false }

    panic!("Unexpected character: {} at {},{}", map[new_pos.y][new_pos.x], new_pos.x, new_pos.y);
}

fn calc_new_position(dir: &Direction, pos: &Point) -> Point {
    match dir {
        Direction::Up    => Point::new(pos.x, pos.y - 1),
        Direction::Right => Point::new(pos.x + 1, pos.y),
        Direction::Down  => Point::new(pos.x, pos.y + 1),
        Direction::Left  => Point::new(pos.x - 1, pos.y),
    }
}

fn wraparound_calc_new_position(dir: &Direction, pos: &Point, map: &Vec<Vec<char>>) -> Point {
    let mut new_pos = pos.clone();

    // Go on the opposite direction until you hit a ' ', that is your new position
    let opposite_dir = match dir {
        Direction::Up => Direction::Down,
        Direction::Right => Direction::Left,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
    };

    loop {
        let pos = calc_new_position(&opposite_dir, &new_pos);
        if map[pos.y][pos.x] == ' ' { break }
        new_pos = pos;
    }
    new_pos
}
