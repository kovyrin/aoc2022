use std::{fs::read_to_string, collections::HashMap, i32::MAX};
use anyhow::Context;
use num_integer::lcm;
use rustc_hash::FxHashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn manhattan_distance(&self, goal: &Point) -> i32 {
        (goal.y-self.y).abs() + (goal.x-self.x).abs()
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Vortex {
    pos: Point,
    direction: Direction,
}
impl Vortex {
    fn new(x: i32, y: i32, direction: Direction) -> Vortex {
        Vortex { pos: Point { x, y }, direction }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Step {
    pos: Point,
    minute: i32,
}

impl Step {
    fn neighbors(&self) -> Vec<Step> {
        let minute = self.minute + 1;
        vec![
            Step { pos: Point { x: self.pos.x + 1, y: self.pos.y }, minute }, // right
            Step { pos: Point { x: self.pos.x, y: self.pos.y + 1 }, minute }, // down
            Step { pos: Point { x: self.pos.x - 1, y: self.pos.y }, minute }, // up
            Step { pos: Point { x: self.pos.x, y: self.pos.y - 1 }, minute }, // left
            Step { pos: Point { x: self.pos.x, y: self.pos.y }, minute },     // wait
        ]
    }
}

fn main() {
    // If first argument is "real", use the real input file
    // Otherwise, use the test input file
    let input_type = std::env::args().nth(1).unwrap_or(String::default());
    let input_file = if input_type.eq("real") {
        "real-input.txt"
    } else if input_type.eq("mini") {
        "mini-input.txt"
    } else if input_type.starts_with("a") {
        "agubelu-input.txt"
    } else {
        "demo-input.txt"
    };
    println!("Using input file: {}", input_file);

    let input: String = read_to_string(input_file).context("failed to read the data file").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let line1 = lines[0];
    let map_width = line1.len() as i32 - 2;
    let mut map_height = 0;
    let mut vortexes = Vec::new();

    for (y, line) in lines[1..].iter().enumerate() {
        map_height = y as i32;
        for (x, c) in line.chars().enumerate() {
            let x = x as i32 - 1;
            let y = y as i32;
            match c {
                '<' => vortexes.push(Vortex::new(x, y, Direction::West)),
                '>' => vortexes.push(Vortex::new(x, y, Direction::East)),
                '^' => vortexes.push(Vortex::new(x, y, Direction::North)),
                'v' => vortexes.push(Vortex::new(x, y, Direction::South)),
                _ => (),
            }
        }
    }

    // Find start and stop positions (empty cells in first and last rows)
    let start_x = lines.first().unwrap().chars().position(|c| c == '.').unwrap() as i32 - 1;
    let start = Point { x: start_x, y: -1 };

    let end_x = lines.last().unwrap().chars().position(|c| c == '.').unwrap() as i32 - 1;
    let end = Point { x: end_x, y: map_height};

    println!("Map size: {}x{}", map_width, map_height);
    println!("Start: {:?}", start);
    println!("End: {:?}", end);

    let entrance_to_exit = fastest_trip_duration(&start, &end, &vortexes, map_width, map_height, 0);
    // let entrance_to_exit = 264;
    println!("Path duration (entrance to exit): {}", entrance_to_exit);

    let exit_to_entrance = fastest_trip_duration(&end, &start, &vortexes, map_width, map_height, entrance_to_exit);
    println!("Path duration (to pick up snacks): {}", exit_to_entrance);

    let final_exit = fastest_trip_duration(&start, &end, &vortexes, map_width, map_height, exit_to_entrance);
    println!("Total duration: {}", final_exit);
}

fn fastest_trip_duration(start: &Point, goal: &Point, vortexes: &Vec<Vortex>, map_width: i32, map_height: i32, start_min: i32) -> i32 {
    // Cache of vortex states at each minute
    let mut vortexes_at_min: HashMap<i32, Vec<Point>> = HashMap::default();

    // Starting position
    let mut steps_to_consider = vec![Step { pos: start.clone(), minute: start_min }];
    let mut best_result = MAX;

    // Vortex positions repeat at most every vortex_cycle minutes
    let vortex_cycle = lcm(map_height, map_width);
    println!("Vortex cycle: {}", vortex_cycle);

    let mut visited = FxHashSet::default();

    while let Some(step) = steps_to_consider.pop() {
        let next_minute = step.minute + 1;
        if next_minute >= best_result { continue }

        // Simulate vortex movement in the next minute
        let vortexes_now = vortexes_at_min.entry(next_minute % vortex_cycle).or_insert_with(|| {
            calculate_vortexes_at_min(next_minute % vortex_cycle, vortexes, map_height, map_width)
        });

        // Check which direction we can go
        let candidate_steps = step.neighbors();

        // Mark this step as visited
        visited.insert(step.to_owned());

        // Consider each candidate step
        for next in candidate_steps.iter() {
            // Check if we've reached the end
            if next.pos == *goal {
                println!("Reached the end {:?} in {} minutes", next.pos, next.minute);
                best_result = next.minute;
            }

            // Skip candidate if we've already visited it
            if visited.contains(&next) { continue }

            // Skip candidate if it's in a vortex
            if vortexes_now.contains(&next.pos) { continue }

            // Make sure we can move there
            if next.pos != step.pos {
                if next.pos.x < 0 || next.pos.x >= map_width || next.pos.y < 0 || next.pos.y >= map_height {
                    continue;
                }
            }

            // println!("Adding candidate {:?} to steps to consider", candidate);
            steps_to_consider.push(next.to_owned());
            steps_to_consider.sort_by(|a, b|
                b.pos.manhattan_distance(&goal).cmp(&a.pos.manhattan_distance(&goal))
            );
        }
    }

    return best_result;
}

fn calculate_vortexes_at_min(minute: i32, vortexes: &Vec<Vortex>, map_height: i32, map_width: i32) -> Vec<Point> {
    vortexes.iter().map(|v| {
        let mut pos = v.pos.clone();
        match v.direction {
            Direction::North => pos.y = coord_sub(pos.y, minute, map_height),
            Direction::South => pos.y = coord_add(pos.y, minute, map_height),
            Direction::East => pos.x = coord_add(pos.x, minute, map_width),
            Direction::West => pos.x = coord_sub(pos.x, minute, map_width),
        };
        pos
    }).collect()
}

// Substracts change from coord, wrapping around at 0 and going to dimension_limit-1
fn coord_sub(coord: i32, change: i32, dimension_limit: i32) -> i32 {
    let change = change % dimension_limit;
    if coord >= change {
        coord - change
    } else {
        dimension_limit - (change - coord)
    }
}

// Adds change to coord, wrapping around at dimension_limit
fn coord_add(coord: i32, change: i32, dimension_limit: i32) -> i32 {
    (coord + change) % dimension_limit
}

#[cfg(test)]
mod tests {
    // #..x.....#
    //  01234567
    // limit = 8

    #[test]
    fn coord_sub() {
        assert_eq!(super::coord_sub(3, 1, 8), 2);
        assert_eq!(super::coord_sub(3, 2, 8), 1);
        assert_eq!(super::coord_sub(3, 3, 8), 0);
        assert_eq!(super::coord_sub(3, 4, 8), 7);
        assert_eq!(super::coord_sub(3, 5, 8), 6);
        assert_eq!(super::coord_sub(3, 6, 8), 5);
        assert_eq!(super::coord_sub(3, 7, 8), 4);
        assert_eq!(super::coord_sub(3, 8, 8), 3);
        assert_eq!(super::coord_sub(3, 9, 8), 2);
        assert_eq!(super::coord_sub(3, 10, 8), 1);
        assert_eq!(super::coord_sub(3, 11, 8), 0);
    }

    #[test]
    fn coord_add() {
        assert_eq!(super::coord_add(3, 1, 8), 4);
        assert_eq!(super::coord_add(3, 2, 8), 5);
        assert_eq!(super::coord_add(3, 3, 8), 6);
        assert_eq!(super::coord_add(3, 4, 8), 7);
        assert_eq!(super::coord_add(3, 5, 8), 0);
        assert_eq!(super::coord_add(3, 6, 8), 1);
        assert_eq!(super::coord_add(3, 7, 8), 2);
        assert_eq!(super::coord_add(3, 8, 8), 3);
        assert_eq!(super::coord_add(3, 9, 8), 4);
        assert_eq!(super::coord_add(3, 10, 8), 5);
        assert_eq!(super::coord_add(3, 11, 8), 6);
    }
}

// Real tests:
// 217 is too low
// 218 incorrect
// 292 should be correct
//
// Stage2: 816
