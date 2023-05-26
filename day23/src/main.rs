use std::{fs::read_to_string, str::Lines, collections::{HashSet, HashMap}};
use anyhow::Context;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    // If first argument is "real", use the real input file
    // Otherwise, use the test input file
    let input_type = std::env::args().nth(1).unwrap_or(String::default());
    let input_file = if input_type.eq("real") {
        "real-input.txt"
    } else if input_type.eq("mini") {
        "mini-input.txt"
    } else {
        "demo-input.txt"
    };
    println!("Using input file: {}", input_file);

    let input: String = read_to_string(input_file).context("failed to read the data file").unwrap();
    let lines: Lines = input.lines();

    let map = lines.map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    // Find all gnomes (encoded as '#') with their coords
    let mut gnomes = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c.eq(&'#') {
                gnomes.push(Point {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }

    draw_map("Initial state", &gnomes);

    let mut proposed_dirs = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for round in 1..=10 {
        println!("Round {}", round);

        // Make a proposal for each gnome
        let proposals: Vec<Option<Point>> = gnomes.iter().map(|gnome| {
            make_proposal(gnome, &gnomes, &proposed_dirs)
        }).collect();

        let duplicate_proposals = find_duplicates(&proposals);

        println!("Proposals:");
        for (gnome_idx, proposal) in proposals.iter().enumerate() {
            println!("  Gnome {:?}: {:?}", gnomes[gnome_idx], proposal);
            match proposal {
                None => {
                    println!("    - Surrounded, not moving")
                },
                Some(p) if duplicate_proposals.contains(p) => {
                    println!("    - Duplicate proposal, not moving");
                },
                Some(p) => {
                    println!("    + Moving gnome {:?} to {:?}", gnomes[gnome_idx], proposal);
                    gnomes[gnome_idx] = *p;
                }
            }
        }

        // Move the first proposed option to the end of the list
        proposed_dirs.rotate_left(1);
        println!("Proposed dirs after shift: {:?}", proposed_dirs);

        draw_map(format!("End of round {}", round).as_str(), &gnomes);
    }

    // Find a bounding box for the gnomes
    let min_x = gnomes.iter().map(|p| p.x).min().unwrap();
    let max_x = gnomes.iter().map(|p| p.x).max().unwrap();
    let min_y = gnomes.iter().map(|p| p.y).min().unwrap();
    let max_y = gnomes.iter().map(|p| p.y).max().unwrap();

    println!("Map size: {}x{}", max_x - min_x + 1, max_y - min_y + 1);

    // Count empty spaces in the bounding box
    let mut empty_spaces = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if !gnomes.contains(&Point { x, y }) {
                empty_spaces += 1;
            }
        }
    }

    println!("Empty spaces: {}", empty_spaces);
}

fn find_duplicates(proposals: &[Option<Point>]) -> HashSet<&Point> {
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();
    for proposal in proposals.iter() {
        if let Some(p) = proposal {
            if seen.contains(p) {
                duplicates.insert(p);
            } else {
                seen.insert(p);
            }
        }
    }
    duplicates
}

fn any_neighbors(gnome: &Point, gnomes: &Vec<Point>, dir: &Direction) -> bool {
    let neighbors_in_direction = match dir {
        Direction::North => vec![
            Point { x: gnome.x - 1, y: gnome.y - 1 },
            Point { x: gnome.x,     y: gnome.y - 1 },
            Point { x: gnome.x + 1, y: gnome.y - 1 },
        ],
        Direction::East => vec![
            Point { x: gnome.x + 1, y: gnome.y - 1 },
            Point { x: gnome.x + 1, y: gnome.y },
            Point { x: gnome.x + 1, y: gnome.y + 1 },
        ],
        Direction::South => vec![
            Point { x: gnome.x - 1, y: gnome.y + 1 },
            Point { x: gnome.x,     y: gnome.y + 1 },
            Point { x: gnome.x + 1, y: gnome.y + 1 },
        ],
        Direction::West => vec![
            Point { x: gnome.x - 1, y: gnome.y - 1 },
            Point { x: gnome.x - 1, y: gnome.y },
            Point { x: gnome.x - 1, y: gnome.y + 1 },
        ],
    };
    neighbors_in_direction.iter().any(|p| gnomes.contains(p))
}

fn make_proposal(gnome: &Point, gnomes: &Vec<Point>, proposed_dirs: &[Direction]) -> Option<Point> {
    let mut neighbors_by_dir = HashMap::new();
    for dir in proposed_dirs.iter() {
        neighbors_by_dir.insert(dir, any_neighbors(gnome, gnomes, dir));
    }

    // If there are no neighbors in any direction, return None
    if !neighbors_by_dir.values().any(|v| *v) {
        return None;
    }

    // Check if any proposed direction is free and propose that move
    for dir in proposed_dirs.iter() {
        if *neighbors_by_dir.get(dir).unwrap() { continue }
        println!("  Proposing movement of {:?} to the {:?}", gnome, dir);
        return Some(make_a_step(gnome, dir));
    }

    // We're surrounded, so no move is possible
    None
}

fn make_a_step(gnome: &Point, dir: &Direction) -> Point {
    match dir {
        Direction::North => Point { x: gnome.x, y: gnome.y - 1 },
        Direction::East => Point { x: gnome.x + 1, y: gnome.y },
        Direction::South => Point { x: gnome.x, y: gnome.y + 1 },
        Direction::West => Point { x: gnome.x - 1, y: gnome.y },
    }
}

fn draw_map(title: &str, gnomes: &Vec<Point>) {
    println!("== {} ==", title);
    let min_x = gnomes.iter().map(|p| p.x).min().unwrap() - 3;
    let max_x = gnomes.iter().map(|p| p.x).max().unwrap() + 3;
    let min_y = gnomes.iter().map(|p| p.y).min().unwrap() - 3;
    let max_y = gnomes.iter().map(|p| p.y).max().unwrap() + 3;

    for y in min_y..=max_y {
        print!("{}\t", y);
        for x in min_x..=max_x {
            if gnomes.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("----------------------------------------------------------")
}

// correct answer for step 1: 3812
