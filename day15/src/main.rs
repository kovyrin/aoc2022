use std::{fs::read_to_string, str::Lines, ops::Range};
use anyhow::{Context,Result, Ok};
use regex::Regex;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    coord: Point,
    range: i32,
}

impl Sensor {
    fn from_str(line: &str) -> Self {
        let (sensor, beacon) = parse_sensor_and_beacon_locations(line);
        let range = manhattan_range(&sensor, &beacon);
        Sensor { coord: sensor, range: range }
    }

    fn find_blackouts(&self, row: i32) -> Range<i32> {
        let row_to_sensor = (row - self.coord.y).abs();
        let blackout_range_start = self.coord.x - self.range + row_to_sensor;
        let blackout_range_end = self.coord.x + self.range - row_to_sensor;
        blackout_range_start..blackout_range_end
    }
}

fn parse_sensor_and_beacon_locations(input: &str) -> (Point, Point) {
    let re = Regex::new(r"x=(-?\d+), y=(-?\d+)").expect("Failed to compile the regex pattern");
    let mut coordinates = re.captures_iter(input);

    let sensor_captures = coordinates.next().expect("Sensor coordinates not found");
    let sensor_x: i32 = sensor_captures[1].parse().expect("Failed to parse sensor's x coordinate");
    let sensor_y: i32 = sensor_captures[2].parse().expect("Failed to parse sensor's y coordinate");

    let beacon_captures = coordinates.next().expect("Beacon coordinates not found");
    let beacon_x: i32 = beacon_captures[1].parse().expect("Failed to parse beacon's x coordinate");
    let beacon_y: i32 = beacon_captures[2].parse().expect("Failed to parse beacon's y coordinate");

    (Point { x: sensor_x, y: sensor_y }, Point { x: beacon_x, y: beacon_y })
}

fn manhattan_range(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn main() -> Result<()>{
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

    let row: i32 = if input_type.eq("real") { 2000000 } else { 10 };

    let sensors = lines.map(|line| {
        Sensor::from_str(line)
    });

    // Find all sensors that cannot reach the given row
    let sensors_in_range: Vec<Sensor> = sensors.filter(|sensor| {
        (sensor.coord.y - row).abs() < sensor.range
    }).collect();

    let mut blackout_ranges = Vec::default();
    for sensor in sensors_in_range.iter() {
        let blackout_range = sensor.find_blackouts(row);
        blackout_ranges.push(blackout_range);
    }

    println!("Number of unique blackout points: {}", total_values_in_ranges(&mut blackout_ranges));

    Ok(())
}

// Courtesy of ChatGPT
fn total_values_in_ranges(ranges: &mut [std::ops::Range<i32>]) -> i32 {
    ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));

    let mut current_range = ranges[0].clone();
    let mut total_length = 0;

    for range in &ranges[1..] {
        if range.start <= current_range.end {
            // Ranges overlap, merge them
            current_range.end = range.end.max(current_range.end);
        } else {
            // Ranges do not overlap, add the current range's length to the total
            total_length += current_range.end - current_range.start;
            current_range = range.clone();
        }
    }

    // Don't forget to add the final range's length
    total_length += current_range.end - current_range.start;

    total_length
}
