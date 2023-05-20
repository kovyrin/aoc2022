use std::{fs::read_to_string, str::Lines, collections::HashSet};
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
    let input_file = if std::env::args().nth(1).unwrap_or(String::default()).eq("real") {
        "real-input.txt"
    } else {
        "demo-input.txt"
    };
    println!("Using input file: {}", input_file);

    let input: String = read_to_string(input_file).context("failed to read the data file").unwrap();
    let mut lines: Lines = input.lines();

    let row: i32 = lines.next().expect("read row line")
                        .split_whitespace().nth(1).expect("get rows")
                        .parse().expect("parse rows count");

    let mut total_sensors = 0;
    let sensors = lines.map(|line| {
        let (sensor, beacon) = parse_sensor_and_beacon_locations(line);
        let range = manhattan_range(&sensor, &beacon);
        let sensor = Sensor { coord: sensor, range: range };
        println!("Parsed sensor = {:?} ", sensor);
        total_sensors += 1;
        sensor
    });

    // Find all sensors that cannot reach the given row
    let sensors_in_range: Vec<Sensor> = sensors.filter(|sensor| {
        (sensor.coord.y - row).abs() < sensor.range
    }).collect();

    println!("Sensors in range from row {} (out of {}):", row, total_sensors);
    for sensor in sensors_in_range.iter() {
        println!("- {:?}", sensor);
    }

    let mut blackout_x: HashSet<i32> = HashSet::default();
    for sensor in sensors_in_range.iter() {
        find_sensor_blackouts(sensor, row, &mut blackout_x);
    }

    println!("Number of unique blackout points: {}", blackout_x.len());

    Ok(())
}

fn find_sensor_blackouts(sensor: &Sensor, row: i32, blackout_x: &mut HashSet<i32>) {
    println!("Analyzing sensor: {:?}", sensor);

    let row_to_sensor = (row - sensor.coord.y).abs();
    println!("Row to sensor distance: {row_to_sensor}");

    let blackout_range_start = sensor.coord.x - sensor.range + row_to_sensor;
    let blackout_range_end = sensor.coord.x + sensor.range - row_to_sensor;
    println!("Blackout range: {}..{}", blackout_range_start, blackout_range_end);

    for x in blackout_range_start..blackout_range_end {
        blackout_x.insert(x);
    }
}
