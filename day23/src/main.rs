use std::{fs::read_to_string, str::Lines};
use anyhow::Context;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

    let mut proposed_dirs = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    let mut round = 1;
    loop {
        if round == 11 {
            // Find a bounding box for the gnomes
            let min_x = gnomes.iter().map(|p| p.x).min().unwrap();
            let max_x = gnomes.iter().map(|p| p.x).max().unwrap();
            let min_y = gnomes.iter().map(|p| p.y).min().unwrap();
            let max_y = gnomes.iter().map(|p| p.y).max().unwrap();

            // Count empty spaces in the bounding box
            let mut empty_spaces = 0;
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if !gnomes.contains(&Point { x, y }) {
                        empty_spaces += 1;
                    }
                }
            }

            println!("Empty spaces on round 10: {}", empty_spaces);
        }

        let gnomes_set = gnomes.iter().collect::<FxHashSet<&Point>>();

        // Make a proposal for each gnome
        let mut proposals: Vec<Option<Point>> = Vec::with_capacity(gnomes.len());
        let mut proposal_count = 0;
        for gnome in gnomes.iter() {
            let proposal = make_proposal(gnome, &gnomes_set, &proposed_dirs);
            if proposal.is_some() {
                proposal_count += 1;
            }
            proposals.push(proposal);
        }

        // If no gnome is moving, we're done
        if proposal_count == 0 { break }

        let duplicate_proposals = find_duplicates(&proposals);
        for (gnome_idx, proposal) in proposals.iter().enumerate() {
            if let Some(p) = proposal {
                if !duplicate_proposals.contains(p) {
                    gnomes[gnome_idx] = p.to_owned();
                }
            }
        }

        // Move the first proposed option to the end of the list
        proposed_dirs.rotate_left(1);

        round += 1;
        if round % 10 == 0 {
            println!("Round {} complete...", round);
        }
    }

    println!("Finished after {} rounds", round);
}

fn find_duplicates(proposals: &[Option<Point>]) -> FxHashSet<&Point> {
    let mut seen = FxHashSet::default();
    let mut duplicates = FxHashSet::default();
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

fn any_neighbors(gnome: &Point, gnomes: &FxHashSet<&Point>, dir: &Direction) -> bool {
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

fn make_proposal(gnome: &Point, gnomes: &FxHashSet<&Point>, proposed_dirs: &[Direction]) -> Option<Point> {
    let mut neighbors_by_dir = FxHashMap::default();
    let mut found_neighbors = false;

    for dir in proposed_dirs.iter() {
        let neighbors = any_neighbors(gnome, gnomes, dir);
        neighbors_by_dir.insert(dir, neighbors);
        found_neighbors = found_neighbors || neighbors;
    }

    // If there are no neighbors in any direction, stay put
    if !found_neighbors { return None }

    // Check if any proposed direction is free and propose that move
    for dir in proposed_dirs.iter() {
        if !neighbors_by_dir.get(dir).unwrap() {
            return Some(take_a_step(gnome, dir))
        }
    }

    // We're surrounded, so no move is possible
    None
}

fn take_a_step(gnome: &Point, dir: &Direction) -> Point {
    match dir {
        Direction::North => Point { x: gnome.x, y: gnome.y - 1 },
        Direction::East => Point { x: gnome.x + 1, y: gnome.y },
        Direction::South => Point { x: gnome.x, y: gnome.y + 1 },
        Direction::West => Point { x: gnome.x - 1, y: gnome.y },
    }
}

// correct answer for step 1: 3812
// correct answer for step 2: 1003
