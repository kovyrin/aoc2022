use std::{str::Lines, fs::read_to_string};
use anyhow::Context;
use regex::Regex;

#[derive(Debug)]
struct RobotCost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_bot: RobotCost,
    clay_bot: RobotCost,
    obsidian_bot: RobotCost,
    geode_bot: RobotCost,
}

impl Blueprint {

    fn from_str(line: &str) -> Self {
        // Blueprint 1:
        // Each ore robot costs 4 ore.
        // Each clay robot costs 2 ore.
        // Each obsidian robot costs 3 ore and 14 clay.
        // Each geode robot costs 2 ore and 7 obsidian.
        let re = Regex::new(r"\d+").unwrap();
        let mut matches = re.find_iter(line);

        let id = matches.next().unwrap().as_str().parse().unwrap();
        let ore_bot_ore = matches.next().unwrap().as_str().parse().unwrap();
        let clay_bot_ore = matches.next().unwrap().as_str().parse().unwrap();
        let obsidian_bot_ore = matches.next().unwrap().as_str().parse().unwrap();
        let obsidian_bot_clay = matches.next().unwrap().as_str().parse().unwrap();
        let geode_bot_ore = matches.next().unwrap().as_str().parse().unwrap();
        let geode_bot_obsidian = matches.next().unwrap().as_str().parse().unwrap();

        Blueprint {
            id,
            ore_bot: RobotCost { ore: ore_bot_ore, clay: 0, obsidian: 0 },
            clay_bot: RobotCost { ore: clay_bot_ore, clay: 0, obsidian: 0 },
            obsidian_bot: RobotCost { ore: obsidian_bot_ore, clay: obsidian_bot_clay, obsidian: 0 },
            geode_bot: RobotCost { ore: geode_bot_ore, clay: 0, obsidian: geode_bot_obsidian },
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

    let mut blueprints = Vec::new();
    for line in lines {
        blueprints.push(Blueprint::from_str(line));
    }

    for bp in blueprints {
        println!("{:?}", bp);
    }
}
