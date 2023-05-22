use std::{collections::{HashMap, HashSet}, str::Lines, fs::read_to_string};
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
struct Volcano {
    valves: HashMap<String, Valve>,
    best_flow: usize,
}

#[derive(Debug)]
struct Invariant {
    current_cave_name: String,
    path: Vec<String>,
    open_valves: HashSet<String>,
    cumulative_flow: usize,
    steps_remaining: i16
}

impl Invariant {
    fn is_current_valve_closed(&self) -> bool {
        !self.open_valves.contains(&self.current_cave_name)
    }
}

impl Volcano {
    fn from_lines(lines: Lines) -> Self {
        let mut volcano = Volcano { valves: HashMap::new(), best_flow: 0 };
        for line in lines {
            let (name, valve) = Valve::from_str(line);
            volcano.valves.insert(name, valve);
        }
        volcano
    }

    fn total_flow(&self, open_valves: &HashSet<String>) -> usize {
        open_valves.iter().map(|v| self.valves[v].flow_rate).sum()
    }

    fn walk_caves(&mut self, step: Invariant) {
        println!("Step:");
        println!("- path: {:?}", step.path);
        println!("- open: {:?}", step.open_valves);

        if step.steps_remaining < 0 { return }
        if step.steps_remaining == 0 {
            if step.cumulative_flow > self.best_flow {
                println!("Found the best flow of {} with path {:?}", step.cumulative_flow, step.path);
                self.best_flow = step.cumulative_flow;
            }
            return
        }

        let current_cave_name = step.current_cave_name.clone();
        let current_valve = (*self.valves.get(&current_cave_name).expect("cave lookup")).clone();

        let cumulative_flow = self.total_flow(&step.open_valves) + step.cumulative_flow;
        let is_fully_open = self.is_fully_open(&step);

        if is_fully_open {
            println!("Fully open!");
        }

        if step.is_current_valve_closed() || current_valve.flow_rate > 0 {
            // Open the current valve
            let mut open_valves = step.open_valves.clone();
            open_valves.insert(step.current_cave_name.clone());

            // Calculate cumulative flow for next step
            let cumulative_flow = cumulative_flow + current_valve.flow_rate;

            if !is_fully_open {
                for cave in current_valve.connections.iter() {
                    let mut path = step.path.clone();
                    path.push(cave.clone());

                    let next_step = Invariant {
                        current_cave_name: cave.clone(),
                        path,
                        open_valves: open_valves.clone(),
                        cumulative_flow,
                        steps_remaining: step.steps_remaining - 2
                    };

                    self.walk_caves(next_step);
                }
            }
        }

        // Try walking the graph without touching the valve
        if !is_fully_open {
            for cave in current_valve.connections.iter() {
                let mut path = step.path.clone();
                path.push(cave.clone());

                let next_step = Invariant {
                    current_cave_name: cave.clone(),
                    path,
                    open_valves: step.open_valves.clone(),
                    cumulative_flow,
                    steps_remaining: step.steps_remaining - 1
                };

                self.walk_caves(next_step);
            }
        }
    }

    fn find_maximum_pressure(&mut self) {
        let start_cave = "AA".to_string();
        self.walk_caves(
            Invariant {
                current_cave_name: start_cave.clone(),
                path: vec![start_cave],
                open_valves: HashSet::new(),
                steps_remaining: 30,
                cumulative_flow: 0,
            }
        );
    }

    fn is_fully_open(&self, step: &Invariant) -> bool {
        for valve in self.valves.keys() {
            if !step.open_valves.contains(valve) {
                return false
            }
        }
        true
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

    volcano.find_maximum_pressure();

    // for (i, (name, valve)) in valves.iter().enumerate() {
    //     println!("Valve {} flow rate {} connected to {:?}", name, valve.flow_rate, valve.connections);
    // }


}
