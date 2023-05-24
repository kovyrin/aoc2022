use std::{str::Lines, fs::read_to_string, collections::VecDeque};
use anyhow::Context;

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

impl Jet {
    fn from_char(c: char) -> Self {
        match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            c => panic!("Unexpected character: '{c}'")
        }
    }
}

#[derive(Debug, Clone)]
struct Rock {
    shape: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct FallingRock {
    rock: Rock,
    col: usize,
    row: usize,
}

#[derive(Debug)]
struct Chamber {
    field: VecDeque<Vec<char>>,
    floor: usize,
    highest_point: usize,
    rock: Option<FallingRock>,
}

impl Chamber {
    fn new() -> Self {
        Chamber {
            field: VecDeque::from(vec![vec!['.'; 7]; 10]),
            highest_point: 0,
            floor: 0,
            rock: None
        }
    }

    fn draw_rock(&mut self, c: char) {
        let falling_rock = self.rock.as_ref().unwrap();
        let rock = &falling_rock.rock;
        for row in 0..rock.height {
            for col in 0..rock.width {
                if rock.shape[rock.height - row - 1][col] != '.' {
                    // println!("Drawing pixel {},{}", falling_rock.col + col, falling_rock.row + row);
                    self.field[falling_rock.row + row - self.floor][falling_rock.col + col] = c;
                }
            }
        }
    }

    fn drop_rock(&mut self, rock: &Rock) {
        let falling_rock = FallingRock {
            rock: rock.clone(),
            col: 2,
            row: self.highest_point + 3,
        };
        self.rock = Some(falling_rock);
        self.draw_rock('@');
    }

    fn apply_jet(&mut self, jet: &Jet) {
        // erase the rock
        self.draw_rock('.');

        match jet {
            Jet::Left => {
                self.maybe_move_rock(-1)
            },
            Jet::Right => {
                self.maybe_move_rock(1)
            },
        }

        // bring the rock back
        self.draw_rock('@');
    }

    fn maybe_move_rock(&mut self, shift: i32) {
        if let Some(falling_rock) = self.rock.as_ref() {
            let new_col = falling_rock.col as i32 + shift;
            if !self.no_collisions(new_col, falling_rock.row as i32) { return }
            self.rock.as_mut().unwrap().col = new_col as usize;
        }
    }

    fn maybe_move_rock_down(&mut self) {
        if self.rock.is_none() { return }
        // erase the rock
        self.draw_rock('.');

        let falling_rock = self.rock.as_ref().unwrap();
        let height = falling_rock.rock.height;
        let rock_row = falling_rock.row;

        if self.no_collisions(falling_rock.col as i32, rock_row as i32 - 1) {
            self.rock.as_mut().unwrap().row -= 1;
            self.draw_rock('@'); // bring the rock back
            return;
        }

        self.draw_rock('#'); // rock has come to rest
        if self.highest_point < rock_row + height {
            self.highest_point = rock_row + height;
            // println!("New highest point: {}", self.highest_point);
            let desired_field_height = self.highest_point + 10 - self.floor;
            let need_rows = desired_field_height - self.field.len();
            for _ in 0..need_rows { self.field.push_back(vec!['.';7]); }
            while self.field.len() > 100 {
                self.field.pop_front();
                self.floor += 1;
            }
        }
        self.rock = None;
    }

    fn no_collisions(&self, new_col: i32, new_row: i32) -> bool {
        if let Some(falling_rock) = self.rock.as_ref() {
            let rock = &falling_rock.rock;
            if new_col < 0 || new_col + rock.width as i32 > 7 || new_row < 0 {
                return false;
            }

            for row in 0..rock.height {
                for col in 0..rock.width {
                    if rock.shape[rock.height - row - 1][col] == '.' { continue }
                    if self.field[new_row as usize + row - self.floor][new_col as usize + col] != '.' {
                        return false;
                    }
                }
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
    let mut lines: Lines = input.lines();
    let jets: Vec<Jet> = lines.next().expect("reading jets").chars().map(|c| Jet::from_char(c)).collect();

    let mut rocks = Vec::new();
    rocks.push(Rock {
        width: 4,
        height: 1,
        shape: vec![
            vec!['@','@','@','@']
        ]
    });

    rocks.push(Rock {
        width: 3,
        height: 3,
        shape: vec![
            vec!['.','@','.'],
            vec!['@','@','@'],
            vec!['.','@','.'],
        ]
    });

    rocks.push(Rock {
        width: 3,
        height: 3,
        shape: vec![
            vec!['.','.','@'],
            vec!['.','.','@'],
            vec!['@','@','@'],
        ]
    });

    rocks.push(Rock {
        width: 1,
        height: 4,
        shape: vec![
            vec!['@'],
            vec!['@'],
            vec!['@'],
            vec!['@'],
        ]
    });

    rocks.push(Rock {
        width: 2,
        height: 2,
        shape: vec![
            vec!['@','@'],
            vec!['@','@'],
        ]
    });

    let mut chamber = Chamber::new();

    let mut rocks_count = 0;
    let mut rock_idx = 0;
    let mut jet_idx = 0;
    let mut last_highest_point = 0;
    let mut height_steps: Vec<usize> = Vec::new();

    loop {
        // If there is no active rock, drop another one
        if chamber.rock.is_none() {
            let rock = &rocks[rock_idx];
            chamber.drop_rock(rock);
            rocks_count += 1;
            rock_idx = (rock_idx + 1) % rocks.len();
            let step = chamber.highest_point - last_highest_point;
            height_steps.push(step);
            last_highest_point = chamber.highest_point;
            if rocks_count > 2022 {
                println!("Highest point after 2022 steps: {}", chamber.highest_point);
                break;
            }
        }

        // Apply jet to the falling rock, potentially moving it
        let jet = &jets[jet_idx];
        chamber.apply_jet(jet);
        jet_idx = (jet_idx + 1) % jets.len();

        // Try moving the rock down
        chamber.maybe_move_rock_down();
    }

    // println!("Steps: {:?}", height_steps);
    let pattern = find_step_pattern(&mut height_steps);
    let pattern_height: usize = pattern.iter().sum();
    println!("Pattern of {} items (total height: {}): {:?}", pattern.len(), pattern_height, pattern);

    const STEP2_ROCK_LIMIT: usize = 1_000_000_000_000;
    let remaining_rocks = STEP2_ROCK_LIMIT - rocks_count;
    println!("Remaining rocks: {}", remaining_rocks);

    let remaining_pattern_count = remaining_rocks / pattern.len();
    println!("Need to repeat the pattern {} times", remaining_pattern_count);

    let tail_len = remaining_rocks % pattern.len();
    println!("After repeating the pattern, we will need {} more rocks", tail_len);

    let tail_height: usize = pattern[0..=tail_len].iter().sum();
    println!("Height of the additional {} rocks ({:?}) will be {}", tail_len, &pattern[0..=tail_len], tail_height);

    let total_height = chamber.highest_point + remaining_pattern_count * pattern_height + tail_height;
    println!("Total height: {}", total_height);
 }

fn find_step_pattern(steps: &mut [usize]) -> &[usize] {
    steps.reverse();

    const CHECK_LEN: usize = 16;
    let beginning_pattern = &steps[0..CHECK_LEN];
    for i in CHECK_LEN..steps.len() {
        let subset = &steps[i..i+CHECK_LEN];
        if subset == beginning_pattern {
            let final_pattern = &mut steps[0..i];
            final_pattern.reverse();
            return final_pattern;
        }
    }

    panic!("Didn't find a pattern");
}

// Real checks
// 1537175792497 - too high
// 1537175792495 - correct
