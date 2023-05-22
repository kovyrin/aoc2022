use std::{collections::{HashMap, VecDeque}, str::Lines, fs::read_to_string, hash::Hash};
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
    distance_between: HashMap<String, HashMap<String, usize>>,
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

    fn calculate_distances_between_caves(&mut self) {
        for start in self.valves.keys() {
            if !self.distance_between.contains_key(start) {
                self.distance_between.insert(start.clone(), HashMap::default());
            }
            let distance_from_start = self.distance_between.get_mut(start).expect("distance from start");
            distance_from_start.insert(start.clone(), 0);

            while self.valves.len() != distance_from_start.len() {
                for (name, valve) in self.valves.iter() {
                    if distance_from_start.contains_key(name) { continue };
                    let dist_from_start = valve.connections.iter()
                        .filter(|v| distance_from_start.contains_key(*v))
                        .map(|v| distance_from_start.get(v).expect("get neighbor distance"))
                        .min();
                    if dist_from_start.is_some() {
                        distance_from_start.insert(name.clone(), *dist_from_start.unwrap() + 1);
                    }
                }
            }
        }
    }

    // fn calculate_optimal_opening_order(&self) -> VecDeque<(String, f32)> {
    //     let mut valves = Vec::default();
    //     for (name, valve) in self.valves.iter() {
    //         if valve.flow_rate > 0 {
    //             let dist = *self.distance_from_start.get(name).expect("distance load");
    //             let cost_of_flow = valve.flow_rate as f32 / dist as f32;
    //             valves.push((name.clone(), cost_of_flow));
    //         }
    //     }
    //     valves.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    //     VecDeque::from(valves)
    // }

    fn simulate(&self, opening_order: Vec<&str>) {
        println!("Simulating path: {:?}", opening_order);

        let mut valve_order: VecDeque<String> = opening_order.iter().map(|s| s.to_string()).collect();
        let mut current_cave = "AA".to_string();
        let mut minute = 1;
        let mut flow_per_min: usize = 0;
        let mut released = 0;

        while minute <= 30 {
            println!("Minute: {}", minute);

            match valve_order.pop_front() {
                Some(dest) => {
                    let distances_from_cur = self.distance_between.get(&current_cave).expect("dist from cur");
                    let dist_to_dest = distances_from_cur.get(&dest).expect("dist to dest");
                    let time_for_action = dist_to_dest + 1;

                    minute += time_for_action;
                    released += flow_per_min * time_for_action;
                    current_cave = dest;
                    flow_per_min += self.valves.get(&current_cave).expect("valve fetch").flow_rate;
                },
                None => {
                    released += flow_per_min;
                    minute += 1;
                }
            }
        }
        println!("Total release: {}\n", released);
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

    volcano.calculate_distances_between_caves();

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

    // let path: VecDeque<(String, f32)> = volcano.calculate_optimal_opening_order();
    // println!("Opening order:");
    // for (valve, cost) in path.iter() {
    //     println!("- {valve} ({cost})");
    // }

    volcano.simulate(vec!["DD", "BB", "JJ", "HH", "EE", "CC"]);
    volcano.simulate(vec!["DD", "JJ", "BB", "HH", "EE", "CC"]);
}
