use std::{collections::{HashMap, VecDeque}, str::Lines, fs::read_to_string, hash::Hash, usize::MAX};
use anyhow::Context;
use regex::Regex;

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: usize,
    connections: Vec<String>,
}

impl Valve {
    fn from_str(line: &str) -> (String, Self) {
        let re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").expect("regex init");
        println!("Parsing: {}", line);
        let valve_cap = re.captures(line).expect("capture valve def");
        let name = valve_cap[1].to_string();
        let flow_rate = valve_cap[2].parse().expect("parse flow rate");
        let connections = valve_cap[3].split(", ").map(|s| s.to_string()).collect();
        (name, Valve { flow_rate, connections })
    }
}
#[derive(Debug, Default)]
struct Volcano {
    valves: HashMap<String, Valve>,
    working_valves: Vec<String>,
    distance_between: HashMap<String, HashMap<String, usize>>,
}

impl Volcano {
    fn from_lines(lines: Lines) -> Self {
        let mut volcano = Volcano::default();
        for line in lines {
            let (name, valve) = Valve::from_str(line);
            if valve.flow_rate > 0 {
                volcano.working_valves.push(name.clone());
            }
            volcano.valves.insert(name, valve);
        }
        volcano
    }

    fn calculate_distances_between_all_caves(&mut self) {
        for start in self.valves.keys() {
            let distance_from_start = self.distance_between.entry(start.clone()).or_default();
            distance_from_start.insert(start.clone(), 0);

            while self.valves.len() != distance_from_start.len() {
                for (name, valve) in self.valves.iter() {
                    let known_distance_len = distance_from_start.get(name).unwrap_or(&MAX);
                    let dist_from_start = valve.connections
                        .iter()
                        .filter_map(|v| distance_from_start.get(v))
                        .min();
                    if let Some(&min_dist_from_start) = dist_from_start {
                        let new_min_dist = min_dist_from_start + 1;
                        if new_min_dist < *known_distance_len {
                            distance_from_start.insert(name.to_owned(),  new_min_dist);
                        }
                    }
                }
            }
        }
    }

    fn simulate(&self, opening_order: Vec<&str>) -> usize {
        println!("Simulating path: {:?}", opening_order);

        let mut valve_order: VecDeque<String> = opening_order.iter().map(|s| s.to_string()).collect();
        let mut current_cave = "AA".to_string();
        let mut minute = 1;
        let mut flow_per_min: usize = 0;
        let mut released = 0;
        let mut total_open = 0;

        while minute <= 30 {
            match valve_order.pop_front() {
                Some(dest) => {
                    let distances_from_cur = self.distance_between.get(&current_cave).expect("dist from cur");
                    let dist_to_dest = distances_from_cur.get(&dest).expect("dist to dest");
                    let time_for_action = dist_to_dest + 1;

                    if minute + time_for_action > 30 { break }

                    minute += time_for_action;
                    released += flow_per_min * time_for_action;

                    println!("Opening {} on minute {}", dest, minute);
                    total_open += 1;
                    current_cave = dest;
                    flow_per_min += self.valves.get(&current_cave).expect("valve fetch").flow_rate;
                },
                None => { break }
            }
        }

        released += flow_per_min * (30 - minute + 1);

        println!("Total release after opening {} valves: {}\n", total_open, released);
        released
    }

    fn find_best_release(&self) -> usize {
        let mut best_release = 0;
        best_release += 1;
        best_release
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
    let mut volcano = Volcano::from_lines(lines);

    volcano.calculate_distances_between_all_caves();
    println!("Distances between caves:");
    for (start, distances_to) in volcano.distance_between.iter() {
        if !start.eq("AA") && volcano.valves.get(start).unwrap().flow_rate == 0 { continue }
        println!("Distances from {start}:");
        for (end, distance) in distances_to.iter() {
            if volcano.valves.get(end).unwrap().flow_rate == 0 { continue }
            println!(" - to {}: {}", end, distance);
        }
        println!();
    }
    println!();

    let start = "AA".to_string();
    let distances_from_start = volcano.distance_between.get(&start).unwrap();
    println!("Distances from {start}:");
    for (end, distance) in distances_from_start.iter() {
        if volcano.valves.get(end).unwrap().flow_rate == 0 { continue }
        println!(" - to {}: {}", end, distance);
    }
    println!();

    let best_release = volcano.find_best_release();
    println!("Best release value found: {best_release}")

    // volcano.simulate(vec!["DD", "BB", "JJ", "HH", "EE", "CC"]);
    // volcano.simulate(vec!["DD", "JJ", "BB", "HH", "EE", "CC"]);
}

// real range:
// - 1339 is too low
// - 1525 is too high
