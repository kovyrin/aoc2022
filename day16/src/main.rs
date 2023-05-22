use std::{collections::{HashMap, VecDeque, HashSet}, str::Lines, fs::read_to_string, usize::MAX};
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
    working_valves: HashSet<String>,
    distance_between: HashMap<String, HashMap<String, usize>>,
}

#[derive(Debug)]
struct Invariant {
    current_cave: String,
    path: Vec<String>,
    unopened_valves: HashSet<String>,
    minute: usize,
    flow_per_min: usize,
    released: usize,
}

impl Volcano {
    fn from_lines(lines: Lines) -> Self {
        let mut volcano = Volcano::default();
        for line in lines {
            let (name, valve) = Valve::from_str(line);
            if valve.flow_rate > 0 {
                volcano.working_valves.insert(name.clone());
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
        let start = "AA".to_string();
        let start_invariant = Invariant {
            current_cave: start.clone(),
            path: vec![start],
            unopened_valves: self.working_valves.clone(),
            minute: 1,
            flow_per_min: 0,
            released: 0,
        };

        self.walk_the_caves(start_invariant, &mut best_release);
        best_release
    }

    fn walk_the_caves(&self, i: Invariant, best_release: &mut usize) {
        for next_name in i.unopened_valves.iter() {
            let distances_from_cur = self.distance_between.get(&i.current_cave).expect("dist from cur");
            let dist_to_dest = distances_from_cur.get(next_name).expect("dist to dest");
            let time_for_step = dist_to_dest + 1;

            // Cannot take this step, it will take more than 30 min to finish
            if i.minute + time_for_step > 30 { continue }

            let remaining_unopened = i.unopened_valves.iter().filter(|v| !v.eq(&next_name)).map(|x| x.clone()).collect();
            let next_flow = self.valves.get(next_name).expect("valve fetch").flow_rate;

            let mut new_path = i.path.clone();
            new_path.push(next_name.clone());

            let next_step = Invariant {
                current_cave: next_name.clone(),
                path: new_path,
                unopened_valves: remaining_unopened,
                minute: i.minute + time_for_step,
                flow_per_min: i.flow_per_min + next_flow,
                released: i.released + i.flow_per_min * time_for_step,
            };
            self.walk_the_caves(next_step, best_release);
        }

        let total_release = i.released + (30 - i.minute + 1) * i.flow_per_min;
        if total_release > *best_release {
            println!("Best new path: {:?}", i.path);
            *best_release = total_release;
        }
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
}

// real range:
// - 1339 is too low
// - 1488 is valid
// - 1525 is too high
