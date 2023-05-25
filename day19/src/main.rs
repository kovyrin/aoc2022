use std::{str::Lines, fs::read_to_string, collections::HashSet, time::Instant, cmp::max};
use anyhow::Context;
use regex::Regex;

const MAX_MINUTES: usize = 24;

#[derive(Debug, Clone)]
struct RobotCost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: usize,

    ore_bot: RobotCost,
    clay_bot: RobotCost,
    obsidian_bot: RobotCost,
    geode_bot: RobotCost,

    max_ore_needed: usize,
    max_clay_needed: usize,
    max_obsidian_needed: usize,
}

enum BuildPlan {
    OreBot,
    ClayBot,
    ObsidianBot,
    GeodeBot,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Invariant {
    minute: usize,
    ore_stock: usize,
    clay_stock: usize,
    obsidian_stock: usize,
    geodes_stock: usize,
    ore_bots: usize,
    clay_bots: usize,
    obsidian_bots: usize,
    geode_bots: usize,
}

impl Invariant {
    fn can_build(&self, bot: &RobotCost) -> bool {
        self.ore_stock >= bot.ore && self.clay_stock >= bot.clay && self.obsidian_stock >= bot.obsidian
    }

    fn build_step(&mut self, plan: BuildPlan, cost: &RobotCost) -> Invariant {
        let mut next_step = self.collect_resources();
        next_step.execute_plan(cost);
        match plan {
            BuildPlan::OreBot => next_step.ore_bots += 1,
            BuildPlan::ClayBot => next_step.clay_bots += 1,
            BuildPlan::ObsidianBot => next_step.obsidian_bots += 1,
            BuildPlan::GeodeBot => next_step.geode_bots += 1,
        }
        next_step
    }

    fn execute_plan(&mut self, cost: &RobotCost) {
        self.ore_stock -= cost.ore;
        self.clay_stock -= cost.clay;
        self.obsidian_stock -= cost.obsidian;
    }

    fn collect_resources(&mut self) -> Invariant {
        Invariant {
            minute: self.minute + 1,
            ore_stock: self.ore_stock + self.ore_bots,
            clay_stock: self.clay_stock + self.clay_bots,
            obsidian_stock: self.obsidian_stock + self.obsidian_bots,
            geodes_stock: self.geodes_stock + self.geode_bots,
            ore_bots: self.ore_bots,
            clay_bots: self.clay_bots,
            obsidian_bots: self.obsidian_bots,
            geode_bots: self.geode_bots,
        }
    }

    fn optimistic_max_geodes(&self, minutes_remaining: usize) -> usize {
        self.geodes_stock + minutes_remaining * self.geode_bots + minutes_remaining * (minutes_remaining.saturating_sub(1))/2
    }
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

            max_ore_needed: max(ore_bot_ore, max(clay_bot_ore, obsidian_bot_ore)),
            max_clay_needed: obsidian_bot_clay,
            max_obsidian_needed: geode_bot_obsidian,
        }
    }

    // Recursively look for an optimal plan to produce the largest number of geodes
    fn find_optimal_plan(&self) -> usize {
        let mut best_result = 0;

        let first_step = Invariant {
            minute: 0,
            ore_stock: 0,
            clay_stock: 0,
            obsidian_stock: 0,
            geodes_stock: 0,
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0
        };

        let mut steps_to_check = vec![first_step];
        let mut seen_steps = HashSet::new();

        while let Some(mut step) = steps_to_check.pop() {
            if seen_steps.contains(&step) { continue }
            if step.minute > MAX_MINUTES { continue }

            // Check how many geodes we may produce in the remaining minutes if we were to build more bots
            let minutes_remaining = MAX_MINUTES - step.minute;
            if step.optimistic_max_geodes(minutes_remaining) <= best_result { continue }

            if step.geodes_stock > best_result {
                best_result = step.geodes_stock;
            }

            steps_to_check.push(step.collect_resources());

            if minutes_remaining >= 3 && step.can_build(&self.ore_bot) && self.max_ore_needed + step.ore_bots > step.ore_stock {
                steps_to_check.push(step.build_step(BuildPlan::OreBot, &self.ore_bot));
            }
            if minutes_remaining >= 4 && step.can_build(&self.clay_bot) && self.max_clay_needed + step.clay_bots > step.clay_stock {
                steps_to_check.push(step.build_step(BuildPlan::ClayBot, &self.clay_bot));
            }
            if minutes_remaining >= 3 && step.can_build(&self.obsidian_bot) && self.max_obsidian_needed + step.obsidian_bots > step.obsidian_stock {
                steps_to_check.push(step.build_step(BuildPlan::ObsidianBot, &self.obsidian_bot));
            }
            if minutes_remaining >= 2 && step.can_build(&self.geode_bot) {
                steps_to_check.push(step.build_step(BuildPlan::GeodeBot, &self.geode_bot));
            }

            seen_steps.insert(step.to_owned());
        }

        best_result
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

    let mut total_quality_level = 0;
    for bp in blueprints.iter() {
        println!("Simulating blueprint {}", bp.id);
        let time = Instant::now();
        let max_geodes_collected = bp.find_optimal_plan();
        let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;

        let quality_level = bp.id * max_geodes_collected;
        total_quality_level += quality_level;

        println!("Max geodes {} collected for blueprint {} in {} ms. Quality level: {}",
            max_geodes_collected, bp.id, elapsed_ms, quality_level);
    }
    println!("Total quality level: {}", total_quality_level);
}

// real checks:
// 2155 - too low
// 2160 - correct
