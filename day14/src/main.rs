use std::{fs::read_to_string, str::Lines, cmp::{min, max}};
use anyhow::{Context, Result};

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        Map {
            tiles: vec![vec![Tile::Air; width]; height]
        }
    }

    fn draw_lines(&mut self, rock_line: &[Point]) {
        let mut segment_start = &rock_line[0];
        for i in 1..rock_line.len() {
            let segment_end = &rock_line[i];
            self.draw_line(segment_start, segment_end);
            segment_start = segment_end;
        }
    }

    fn draw_line(&mut self, segment_start: &Point, segment_end: &Point) {
        if segment_start.x == segment_end.x {
            self.draw_vertical_line(segment_start, segment_end);
        } else if segment_start.y == segment_end.y {
            self.draw_horizontal_line(segment_start, segment_end);
        } else {
            panic!("Unknown type of line: {:?} to {:?}", segment_start, segment_end);
        }
    }

    fn draw_vertical_line(&mut self, segment_start: &Point, segment_end: &Point) {
        let start_y = min(segment_start.y, segment_end.y);
        let end_y = max(segment_start.y, segment_end.y);
        for y in start_y..=end_y {
            self.tiles[y][segment_start.x] = Tile::Rock;
        }
    }

    fn draw_horizontal_line(&mut self, segment_start: &Point, segment_end: &Point) {
        let start_x = min(segment_start.x, segment_end.x);
        let end_x = max(segment_start.x, segment_end.x);
        for x in start_x..=end_x {
            self.tiles[segment_start.y][x] = Tile::Rock;
        }
    }

    fn simulate_sand(&mut self) -> bool {
        let mut sand = Point { x: 500, y: 0 };
        loop {
            if sand.y == self.tiles.len() - 1 {
                println!("Infinite fall at {:?}", sand);
                return true; // infinite fall
            }
            if self.tiles[sand.y+1][sand.x] == Tile::Air {
                println!("Fall down to {:?}", sand);
                sand.y += 1;
            } else if self.tiles[sand.y+1][sand.x-1] == Tile::Air {
                println!("Fall down-left to {:?}", sand);
                sand.x -= 1;
                sand.y += 1;
            } else if self.tiles[sand.y+1][sand.x+1] == Tile::Air {
                println!("Fall down-right to {:?}", sand);
                sand.x += 1;
                sand.y += 1;
            } else {
                self.tiles[sand.y][sand.x] = Tile::Sand;
                println!("At rest at {:?}", sand);
                return false; // stopped at rest
            }
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
    println!("Using input file: {input_file}");

    let input: String = read_to_string(input_file).context("failed to read the data file")?;
    let lines: Lines = input.lines();

    let mut rock_lines: Vec<Vec<Point>> = Vec::default();

    let mut map_width: usize = 0;
    let mut map_height: usize = 0;

    for line in lines {
        let line = line.replace(" -> ", ">");
        let points: Vec<Point> = line.split(">").map(|coords| {
            let mut coords = coords.split(",").map(|c| c.parse::<usize>());

            let x = coords.next().expect("parse x").unwrap();
            if map_width < x { map_width = x };

            let y = coords.next().expect("parse y").unwrap();
            if map_height < y { map_height = y };

            Point { x, y }
        }).collect();

        rock_lines.push(points);
    }

    let mut map = Map::new(map_width + 1, map_height + 1);
    for rock_line in rock_lines.iter() {
        map.draw_lines(rock_line);
    }

    let mut sand_count = 0;
    loop {
        if map.simulate_sand() {
            break
        } else {
            sand_count += 1;
        }
    }

    println!("Finished after {sand_count} iterations");

    Ok(())
}
