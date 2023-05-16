use std::{fs::read_to_string, str::Lines, collections::HashSet};
use anyhow::{Context,Result, Ok};
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Point {
    fn mov(&mut self, direction: char) {
        match direction {
            'U' => self.y -= 1,
            'D' => self.y += 1,
            'L' => self.x -= 1,
            'R' => self.x += 1,
            c => panic!("Unexpected direction value: {c}")
        }
    }

    fn chase(&mut self, head: &Point) {
        // Same position
        if self.x == head.x && self.y == head.y {
            return;
        }

        // Head in the same column
        if self.x == head.x {
            if self.y - head.y >= 2 { self.y -= 1 } // directly above
            else if head.y - self.y >= 2 { self.y += 1 } // directly below
            return
        }

        // Head in the same row
        if self.y == head.y {
            if self.x - head.x >= 2 { self.x -= 1 }
            else if head.x - self.x >= 2 { self.x += 1 }
            return
        }

        // Move diagonally towards the head
        if (head.x - self.x).abs() > 1 || (head.y - self.y).abs() > 1 {
            let x_step = if head.x > self.x { 1 } else { -1 };
            let y_step = if head.y > self.y { 1 } else { -1 };

            self.x += x_step;
            self.y += y_step;
        }
    }
}

#[derive(Debug)]
struct Rope {
    head: Point,
    tail: Point,
    tail_positions: HashSet<Point>
}

impl Rope {
    fn new() -> Self {
        Rope {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
            tail_positions: HashSet::default(),
        }
    }

    fn mov(&mut self, steps: u32, direction: char) {
        for _ in 0..steps {
            self.head.mov(direction);
            self.tail.chase(&self.head);
            self.tail_positions.insert(self.tail.clone());
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

    let input: String = read_to_string(input_file).context("failed to read the data file").unwrap();
    let lines: Lines = input.lines();

    let mut rope = Rope::new();
    for line in lines {
        let direction = line.chars().next().expect("parsing direction");
        let steps = line[2..].parse::<u32>().expect("parsing step");
        rope.mov(steps, direction);
    }

    println!("Unique positions count: {}", rope.tail_positions.len());

    Ok(())
}
