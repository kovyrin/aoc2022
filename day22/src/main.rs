use std::{fs::read_to_string, str::Lines, cmp::max};
use anyhow::Context;
use Direction::*;

type FlatMap = Vec<Vec<char>>;

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

#[derive(Debug)]
struct Cube {
    size: usize,
    faces: Vec<FlatMap>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Face {
    Top = 0,
    Front = 1,
    Bottom = 2,
    Rear = 3,
    Right = 4,
    Left = 5,
}
impl Face {
    fn take_step(&self, dir: &Direction) -> Face {
        match self {
            Face::Top => match dir {
                Up => Face::Rear,
                Right => Face::Right,
                Down => Face::Front,
                Left => Face::Left,
            },
            Face::Front => match dir {
                Up => Face::Top,
                Right => Face::Right,
                Down => Face::Bottom,
                Left => Face::Left,
            },
            Face::Bottom => match dir {
                Up => Face::Front,
                Right => Face::Right,
                Down => Face::Rear,
                Left => Face::Left,
            },
            Face::Rear => match dir {
                Up => Face::Bottom,
                Right => Face::Right,
                Down => Face::Top,
                Left => Face::Left,
            },
            Face::Right => match dir {
                Up => Face::Top,
                Right => Face::Rear,
                Down => Face::Bottom,
                Left => Face::Front,
            },
            Face::Left => match dir {
                Up => Face::Top,
                Right => Face::Front,
                Down => Face::Bottom,
                Left => Face::Rear,
            },
        }
    }
}

fn demo_faces()-> Vec<(usize, usize)> {
    vec![
        (2, 0), // Top
        (2, 1), // Front
        (2, 2), // Bottom
        (0, 1), // Rear
        (3, 2), // Right
        (1, 1), // Left
    ]
}

fn real_faces()-> Vec<(usize, usize)> {
    todo!()
}

type PositionTransformer = fn(Point, usize) -> Point;
type FaceTransition = (Direction, PositionTransformer);

// Returns an array of adjacent faces, directions after transition, and a function to get the new position
/*
    ..T.
    RlF.
    ..Br
*/
// for a given src_face, returns:
// - the dst_face
// - the the direction of movement on the destination face
// - a function that takes the current position on src_face and returns the new position on the dst_face

fn demo_transition_for_face(src_face: &Face, dst_face: &Face) -> FaceTransition {
    match src_face {
        Face::Top => match dst_face {
            Face::Front => (Down, |pos, ____| Point::new(pos.x, 0)),
            Face::Rear => (Down, |pos, size| Point::new(size - pos.x - 1, 0)),
            Face::Right => (Down, |pos, ____| Point::new(pos.y, 0)),
            Face::Left => (Left, |pos, size| Point::new(size - 1, size - pos.y - 1)),
            _ => panic!("Unexpected destination face: {:?}", dst_face),
        },
        Face::Front => match dst_face  {
            Face::Top    => (Up,   |pos, size| Point::new(pos.x, size - 1)),
            Face::Bottom => (Down, |pos, ____| Point::new(pos.x, 0)),
            Face::Left   => (Left, |pos, size| Point::new(size - 1, pos.y)),
            Face::Right  => (Down, |pos, ____| Point::new(pos.y, 0)),
            _ => panic!("Unexpected destination face: {:?}", dst_face),
        },
        Face::Bottom => match dst_face {
            Face::Front => (Up,    |pos, size| Point::new(pos.x, size - 1)),
            Face::Right => (Right, |pos, ____| Point::new(0, pos.y)),
            Face::Rear  => (Down,  |pos, size| Point::new(size - pos.x - 1, size - 1)),
            Face::Left  => (Up,    |pos, size| Point::new(size - pos.y - 1, size - 1)),
            _ => panic!("Unexpected destination face: {:?}", dst_face),
        },
        Face::Rear => match dst_face {
            Face::Top => (Down, |pos, size| Point::new(size - pos.x - 1, 0)),
            Face::Bottom => (Up, |pos, size| Point::new(pos.x, size - 1)),
            Face::Right => todo!(),
            Face::Left => (Right, |pos, ____| Point::new(0, pos.y)),
            _ => panic!("Unexpected destination face: {:?}", dst_face),
        },
        Face::Right => match dst_face {
            Face::Top => todo!(),
            Face::Front => todo!(),
            Face::Bottom => todo!(),
            Face::Rear => todo!(),
            _ => panic!("Unexpected destination face: {:?}", dst_face),
        },
        Face::Left => match dst_face {
            Face::Top    => (Right, |pos, ____| Point::new(0, pos.x)),
            Face::Front  => (Right, |pos, ____| Point::new(0, pos.y)),
            Face::Bottom => (Right, |pos, size| Point::new(0, size - pos.x - 1)),
            Face::Rear   => (Left,  |pos, size| Point::new(size - 1, pos.y)),
            _ => panic!("Unexpected destination face: {:?}", dst_face),
        },
    }
}

impl Cube {
    fn from_flat_map(map: &FlatMap) -> Self {
        let map_len = map.len();
        let map_width = map.iter().map(|line| line.len()).max().unwrap();
        let face_size = max(map_len, map_width) / 4;

        let mut cube = Cube {
            size: face_size,
            faces: vec![vec![vec![' '; face_size]; face_size]; 6],
        };

        match face_size {
            4 => cube.load_map(map, &demo_faces()),
            50 => cube.load_map(map, &real_faces()),
            _ => panic!("Unexpected face size: {}", face_size),
        }

        return cube
    }

    /*
     ..T.
     RlF.
     ..Br
     */
    fn load_map(&mut self, map: &FlatMap, face_positions: &Vec<(usize, usize)>) {
        for face in Face::Top as usize..=Face::Left as usize {
            let (face_x, face_y) = face_positions[face];
            self.load_face(face, map, face_x, face_y);
        }
    }

    fn load_face(&mut self, face_id: usize, map: &FlatMap, x: usize, y: usize) {
        let face_map: &mut FlatMap = &mut self.faces[face_id];
        for col in 0..self.size {
            for row in 0..self.size {
                let map_x = x * self.size + col + 1;
                let map_y = y * self.size + row + 1;
                face_map[row][col] = map[map_y][map_x];
            }
        }
    }

    fn flat_coordinates(&self, face: Face, pos: Point) -> Point {
        match self.size {
            4 => self.flat_coordinates_internal(face, pos, &demo_faces()),
            50 => self.flat_coordinates_internal(face, pos, &real_faces()),
            _ => panic!("Unexpected face size: {}", self.size),
        }
    }

    fn flat_coordinates_internal(&self, face: Face, pos: Point, face_positions: &Vec<(usize, usize)>) -> Point {
        let (face_x, face_y) = face_positions[face as usize];
        let flat_x = face_x * self.size + pos.x;
        let flat_y = face_y * self.size + pos.y;
        Point::new(flat_x, flat_y)
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

    // Part 1: The map is flat
    let mut pos = start_pos;
    let mut dir = Right;

    // Execute the instructions
    for instruction in instructions.iter() {
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

    // Part 2: The map is a fucking cube 🤯
    // We start in the corner of a specific face
    let mut pos = Point::new(0, 0);
    let mut face = Face::Top;
    let mut dir = Right;

    // Parse the flat map into a cube
    let cube = Cube::from_flat_map(&map);
    println!("Cube loaded with face size: {}", cube.size);

    for instruction in instructions.iter() {
        match instruction.as_str() {
            "R" => { dir.turn_cw() },
            "L" => { dir.turn_ccw() },
            steps => {
                let num_steps = steps.parse().unwrap();
                for _ in 0..num_steps {
                    if !cube_go_forward(&mut dir, &mut pos, &mut face,  &cube) { break }
                }
            }
        }
    }

    println!("Final cube position: pos = {:?}, face = {:?}, direction = {:?}", pos, face, dir);

    let flat_pos = cube.flat_coordinates(face, pos);
    print_results("part 2", &flat_pos, &dir);
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

fn cube_go_forward(dir: &mut Direction, pos: &mut Point, face: &mut Face, cube: &Cube) -> bool {
    // If we are on the edge of the face, so we need to transition to a new face
    let (new_face, new_pos, new_dir) = if pos.x == 0 || pos.y == 0 || pos.x >= cube.size-1 || pos.y >= cube.size-1 {
        cube_take_step(dir, pos, face, cube)
    } else {
        (*face, pos.take_step(dir), *dir)
    };

    // Step into the empty space
    match cube.faces[new_face as usize][new_pos.y][new_pos.x] {
        '.' => {
            *face = new_face;
            *pos = new_pos;
            *dir = new_dir;
            return true;
        },
        '#' => return false,
        _ => panic!("Unexpected character: {} at {},{}", cube.faces[new_face as usize][new_pos.y][new_pos.x], new_pos.x, new_pos.y),
    }
}

// Note: this is only called when the requested step is off the edge of the current face
fn cube_take_step(dir: &Direction, pos: &Point, src_face: &Face, cube: &Cube) -> (Face, Point, Direction) {
    let dst_face = src_face.take_step(dir);
    let (new_dir, f_transform) = match cube.size {
        4 => demo_transition_for_face(src_face, &dst_face),
        50 => todo!(),
        _ => panic!("Unexpected face size: {}", cube.size),
    };

    // Find the transition that will take us to the new face
    let new_pos = f_transform(*pos, cube.size);

    (dst_face, new_pos, new_dir)
}

// Final position for part 1: 3,164 with dir=Left
// Password for part 1 = 1000 * 164 + 4 * 3 + 2 = 164014
