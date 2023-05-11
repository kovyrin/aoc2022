use std::{str::Lines, fs::read_to_string};

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

    for line in lines {
        println!("Analyzing line: {}", line);

        let packet_start = detect_unique_segment_start(line, 4);
        println!("Packet start: {}", packet_start);

        let message_start = detect_unique_segment_start(line, 14);
        println!("Message start: {}", message_start);
    }
  }

fn detect_unique_segment_start(line: &str, segment_len: usize) -> usize {
    let chars = line.as_bytes().to_vec();
    for pos in segment_len..chars.len()-1 {
        let mut segment = chars[pos-segment_len..pos].to_vec();
        segment.sort();

        let mut valid_segment = true;
        for p in 1..segment_len {
            if segment[p-1] == segment[p] {
                valid_segment = false;
                break;
            }
        }
        if valid_segment {
            return pos;
        }
    }
    return 0;
}
