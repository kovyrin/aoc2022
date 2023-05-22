use std::{collections::{HashMap, HashSet}, str::Lines, fs::read_to_string, usize::MAX};
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
    distance_between: HashMap<String, HashMap<String, usize>>,
}

#[derive(Debug)]
struct Invariant {
    current_cave: String,
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
            volcano.valves.insert(name, valve);
        }
        volcano
    }

    fn calculate_distances_between_all_caves(&mut self) {
        for start in self.valves.keys() {
            let distance_from_start = self.distance_between.entry(start.to_owned()).or_default();
            distance_from_start.insert(start.to_owned(), 0);

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

    fn find_best_release(&self) -> usize {
        let mut best_release = 0;
        let start = "AA".to_string();
        let working_valves: HashSet<String> = self.valves.iter()
            .filter(|(_,v)| v.flow_rate > 0)
            .map(|(k,_)| k.to_owned())
            .collect();

        let start_invariant = Invariant {
            current_cave: start.to_owned(),
            unopened_valves: working_valves,
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

            let remaining_unopened = i.unopened_valves.iter().filter(|v| *v != next_name).cloned().collect();
            let next_flow = self.valves.get(next_name).expect("valve fetch").flow_rate;

            let next_step = Invariant {
                current_cave: next_name.to_owned(),
                unopened_valves: remaining_unopened,
                minute: i.minute + time_for_step,
                flow_per_min: i.flow_per_min + next_flow,
                released: i.released + i.flow_per_min * time_for_step,
            };
            self.walk_the_caves(next_step, best_release);
        }

        let total_release = i.released + (30 - i.minute + 1) * i.flow_per_min;
        if total_release > *best_release {
            println!("Best new path with total release of {}", total_release);
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

    let best_release = volcano.find_best_release();
    println!("Best release value found: {best_release}")
}

// real range:
// - 1339 is too low
// - 1488 is valid
// - 1525 is too high
