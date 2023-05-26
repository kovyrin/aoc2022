use std::{fs::read_to_string, str::Lines};
use anyhow::Context;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
    // - Turn left
    // - Go forward 5 spaces
    // - Turn right
    // - Go forward 10 spaces
    // - Turn left
    // - Go forward 4 spaces
    // - Turn right
    // - Go forward 5 spaces
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

    // Execute the instructions
    let mut pos_y = 1;
    let mut pos_x = map[1].iter().position(|&c| c == '.').unwrap();
    let mut dir = Direction::Right;
    for instruction in instructions {
        match instruction.as_str() {
            "R" => {
                println!("Turning right");
                dir = turn_cw(dir);
            },
            "L" => {
                println!("Turning left");
                dir = turn_ccw(dir);
            },
            steps => {
                let num_steps = steps.parse::<usize>().unwrap();
                println!("Going forward {} steps", num_steps);
                for _ in 0..num_steps {
                    if !go_forward(&dir, &mut pos_x, &mut pos_y, &map) {
                        break;
                    }
                }
            }
        }
    }

    println!("Final position: {},{} with dir={:?}", pos_x, pos_y, dir);

    // Calculate the password:
    let dir_coeff = match dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };
    let password = 1000 * pos_y + 4 * pos_x + dir_coeff;
    println!("Password = 1000 * {} + 4 * {} + {} = {}", pos_y, pos_x, dir_coeff, password);
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

fn go_forward(dir: &Direction, pos_x: &mut usize, pos_y: &mut usize, map: &Vec<Vec<char>>) -> bool {
    let (mut new_pos_x, mut new_pos_y) = calc_new_position(dir, *pos_x, *pos_y);

    // If we hit an empty space, that means we need to wrap around to the other side of the map
    if map[new_pos_y][new_pos_x] == ' ' {
        println!("- Hit empty space at {},{}", new_pos_x, new_pos_y);
        (new_pos_x, new_pos_y) = wraparound_calc_new_position(dir, *pos_x, *pos_y, map);
        println!("- Teleported to position: {},{}", new_pos_x, new_pos_y);
    }

    if map[new_pos_y][new_pos_x] == '#' {
        println!("- Hit a wall at {},{}", new_pos_x, new_pos_y);
        return false;
    }

    if map[new_pos_y][new_pos_x] == '.' {
        println!("- Moved to {},{}", new_pos_x, new_pos_y);
        *pos_x = new_pos_x;
        *pos_y = new_pos_y;
        return true;
    }

    panic!("Unexpected character: {} at {},{}", map[new_pos_y][new_pos_x], new_pos_x, new_pos_y);
}

fn calc_new_position(dir: &Direction, pos_x: usize, pos_y: usize) -> (usize, usize) {
    match dir {
        Direction::Up => (pos_x, pos_y - 1),
        Direction::Right => (pos_x + 1, pos_y),
        Direction::Down => (pos_x, pos_y + 1),
        Direction::Left => (pos_x - 1, pos_y),
    }
}

fn wraparound_calc_new_position(dir: &Direction, pos_x: usize, pos_y: usize, map: &Vec<Vec<char>>) -> (usize, usize) {
    let mut new_pos_x = pos_x;
    let mut new_pos_y = pos_y;

    // Go on the opposite direction until you hit a ' ', that is your new position
    let opposite_dir = match dir {
        Direction::Up => Direction::Down,
        Direction::Right => Direction::Left,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
    };

    loop {
        let (x, y) = calc_new_position(&opposite_dir, new_pos_x, new_pos_y);
        if map[y][x] == ' ' { break }
        new_pos_x = x;
        new_pos_y = y;
    }

    (new_pos_x, new_pos_y)
}


// real checks:
// 164022 - too high
