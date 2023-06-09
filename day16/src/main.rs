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
    walkers: Vec<Walker>,
    unopened_valves: HashSet<String>,
    minute: usize,
    flow_per_min: usize,
    released: usize,
}

#[derive(Debug, Clone)]
struct Walker {
    name: String,
    current_cave: String,
    target_cave: Option<String>,
    steps_remaining: usize,
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
            walkers: vec![
                Walker {
                    name: "human".to_string(),
                    current_cave: start.to_owned(),
                    target_cave: None,
                    steps_remaining: 0,
                },
                Walker {
                    name: "elephant".to_string(),
                    current_cave: start.to_owned(),
                    target_cave: None,
                    steps_remaining: 0,
                }
            ],
            unopened_valves: working_valves,
            minute: 1,
            flow_per_min: 0,
            released: 0,
        };

        self.walk_the_caves(start_invariant, &mut best_release, 26);
        best_release
    }

    fn walk_the_caves(&self, i: Invariant, best_release: &mut usize, time_limit: usize) {
        if i.minute > time_limit { return }

        let mut still_unopened = i.unopened_valves;
        let mut next_flow = i.flow_per_min;

        // First, check if there are any walkers that have reached their target on this step
        let mut walkers = i.walkers;
        for walker in walkers.iter_mut() {
            if walker.target_cave.is_none() { continue }
            if walker.steps_remaining == 0 {
                walker.current_cave = walker.target_cave.take().expect("target cave");
                still_unopened.remove(&walker.current_cave);
                next_flow += self.valves.get(&walker.current_cave).expect("valve fetch").flow_rate;
            }
        }

        let total_release = i.released + (time_limit - i.minute + 1) * next_flow;
        if total_release > *best_release {
            println!("Best new path with total release of {} and current flow of {}", total_release, next_flow);
            *best_release = total_release;
        }

        // Get the list of unopened valves and their flow rates, then only pick those we can reach
        let remaining_minutes = time_limit - i.minute;
        let mut unopened_with_rates = still_unopened.iter()
            .map(|v| (v, self.valves.get(v).expect("valve").flow_rate))
            .collect::<Vec<(&String, usize)>>();
        unopened_with_rates.sort_by(|a,b| b.1.cmp(&a.1));
        unopened_with_rates.truncate(remaining_minutes);

        // Project the value of remaining unopened valves in remaining time to see if it makes sense to continue
        let total_potential_value = unopened_with_rates.iter().enumerate().map(|(i,v)| (remaining_minutes - i) * v.1).sum::<usize>();
        if total_potential_value + total_release < *best_release { return }

        // Now, for the first walker than needs a target, we iterate over all unopened valves
        // and generate invariants for each one. When those invariants get processed, the next
        // function call will take care of iterating over other walkers without a target.
        if still_unopened.len() > 0 {
            let active_targets = walkers.iter().filter_map(|w| w.target_cave.clone()).collect::<HashSet<String>>();
            let mut walkers_without_target = walkers.iter().filter(|w| w.target_cave.is_none());
            while let Some(walker) = walkers_without_target.next() {
                for target in unopened_with_rates.iter() {
                    let target = target.0;
                    if active_targets.contains(target) { continue }

                    let distances_from_cur = self.distance_between.get(&walker.current_cave).expect("dist from cur");
                    let dist_to_target = distances_from_cur.get(target).expect("dist to dest");
                    let time_to_target = dist_to_target + 1;

                    // Cannot take this step, it will take more than TIME_LIMIT min to finish
                    if i.minute + time_to_target >= time_limit { continue }

                    let mut new_walkers = walkers.clone();
                    let mut walker = new_walkers.iter_mut().find(|w| w.name == walker.name).expect("walker");
                    walker.target_cave = Some(target.to_owned());
                    walker.steps_remaining = time_to_target;

                    let next_step = Invariant {
                        walkers: new_walkers,
                        unopened_valves: still_unopened.clone(),
                        minute: i.minute,
                        flow_per_min: next_flow,
                        released: i.released,
                    };
                    self.walk_the_caves(next_step, best_release, time_limit);
                }
            }
        }

        // Now we can take a step forward in time
        let mut step_minutes = 1;

        // Calculate the step by finding the walker with the lowest number of steps remaining
        let still_walking = walkers.iter().any(|w| w.steps_remaining > 0);
        if still_walking {
            step_minutes = walkers.iter().filter(|w| w.steps_remaining > 0)
                .map(|w| w.steps_remaining)
                .min()
                .expect("min steps remaining");

            for walker in walkers.iter_mut() {
                if walker.steps_remaining > 0 {
                    walker.steps_remaining -= step_minutes;
                }
            }
        }

        let next_step = Invariant {
            walkers,
            unopened_valves: still_unopened,
            minute: i.minute + step_minutes,
            flow_per_min: next_flow,
            released: i.released + i.flow_per_min + (step_minutes - 1) * next_flow,
        };
        self.walk_the_caves(next_step, best_release, time_limit);
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

// Demo result: 1651
//
// Real data:
// part1: 1488
// part2: 2111
//   Walkers: [
//     Walker { name: "human", path: ["YL", "EA", "JU", "TI", "RH", "PK", "VM"] },
//     Walker { name: "elephant", path: ["IR", "JT", "SQ", "IP", "YD", "UX"] },
//   ]
