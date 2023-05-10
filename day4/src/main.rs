use std::{str::Lines, fs::read_to_string};

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32
}

impl Range {
    fn from_str(range_str: &str) -> Self {
        let mut parts = range_str.split("-");
        let begin_str = parts.next().unwrap();
        let end_str = parts.next().unwrap();

        Range {
            start: begin_str.parse().unwrap(),
            end: end_str.parse().unwrap()
        }
    }

    fn contains(&self, other: &Range) -> bool {
        (self.start <= other.start && self.end >= other.end) || (other.start <= self.start && other.end >= self.end)
    }

    fn intersects(&self, other: &Range) -> bool {
        other.contains(self) ||
        (self.start <= other.start && self.end >= other.start) ||
        (self.start <= other.end && self.end >= other.end)
    }
}

fn main() {
    // If first argument is "real", use the real input file
    // Otherwise, use the test input file
    let args: Vec<String> = std::env::args().collect();
    let input_file: &str;
    if args.len() > 1 && args[1] == "real" {
        input_file = "real-input.txt";
    } else {
        input_file = "demo-input.txt";
    }
    println!("Using input file: {}", input_file);

    let input: String = read_to_string(input_file).unwrap();
    let lines: Lines = input.lines();

    let mut fully_contained = 0;
    let mut intersecting = 0;

    for line in lines {
        let mut ranges = line.split(",");
        let range1 = Range::from_str(ranges.next().unwrap());
        let range2 = Range::from_str(ranges.next().unwrap());

        if range1.contains(&range2) || range2.contains(&range1) {
            fully_contained += 1;
        }

        if range1.intersects(&range2) {
            intersecting += 1;
        }
    }

    println!("Fully contained pairs: {}", fully_contained);
    println!("Intersecting pairs: {}", intersecting);
}
