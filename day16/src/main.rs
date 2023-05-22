use std::{collections::{HashMap, VecDeque}, str::Lines, fs::read_to_string};
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
    distance_from_start: HashMap<String, u16>,
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

    fn calculate_distances_from_start(&mut self) {
        self.distance_from_start.insert("AA".to_string(), 0);
        while self.valves.len() != self.distance_from_start.len() {
            for (name, valve) in self.valves.iter() {
                if self.distance_from_start.contains_key(name) { continue };
                let dist_from_start = valve.connections.iter()
                    .filter(|v| self.distance_from_start.contains_key(*v))
                    .map(|v| self.distance_from_start.get(v).expect("get neighbor distance"))
                    .min();
                if dist_from_start.is_some() {
                    self.distance_from_start.insert(name.clone(), *dist_from_start.unwrap() + 1);
                }
            }
        }
    }

    fn calculate_optimal_opening_order(&self) -> VecDeque<(String, f32)> {
        let mut valves = Vec::default();
        for (name, valve) in self.valves.iter() {
            if valve.flow_rate > 0 {
                let dist = *self.distance_from_start.get(name).expect("distance load");
                let cost_of_flow = valve.flow_rate as f32 / dist as f32;
                valves.push((name.clone(), cost_of_flow));
            }
        }
        valves.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        VecDeque::from(valves)
    }

    fn simulate(&self, opening_order: Vec<&str>) {
        let valve_order: VecDeque<String> = opening_order.iter().map(|s| s.to_string()).collect();
        let current_cave = "AA".to_string();
        let mut minute = 1;
        let mut flow_per_min = 0;
        let mut released = 0;

        while minute <= 30 {
            match valve_order.pop_front() {
                Some(dest) => {
                    released += flow_per_min + self.distance_between[current_cave][dest];
                },
                None => {
                    released += flow_per_min;
                    minute += 1;
                }
            }
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

    volcano.calculate_distances_from_start();

    println!("Distances from start:");
    for (name, dist) in volcano.distance_from_start.iter() {
        println!(" - {name}: {dist}");
    }

    let path: VecDeque<(String, f32)> = volcano.calculate_optimal_opening_order();
    println!("Opening order:");
    for (valve, cost) in path.iter() {
        println!("- {valve} ({cost})");
    }

    volcano.simulate(vec!["DD", "BB", "JJ", "HH", "EE", "CC"]);
}
