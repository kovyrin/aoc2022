use std::{str::Lines, fs::read_to_string};

type Stack = Vec<char>;

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
    let mut lines: Lines = input.lines();

    let mut diagram_lines: Vec<&str> = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break
        }
        diagram_lines.push(line);
    }

    let mut stacks = parse_diagram(&mut diagram_lines);

    for line in lines {
        let mut command_parts = line.split(" ");
        let move_count: usize = command_parts.nth(1).unwrap().parse().unwrap();
        let src_idx: usize = command_parts.nth(1).unwrap().parse().unwrap();
        let dst_idx: usize = command_parts.nth(1).unwrap().parse().unwrap();

        let src = &mut stacks[src_idx-1];
        let mut items_to_move = src.split_off(src.len() - move_count);
        stacks[dst_idx-1].append(&mut items_to_move);
    }

    let mut top_items: Vec<char> = Vec::new();
    for stack in stacks {
        top_items.push(*stack.last().unwrap());
    }

    let result: String = top_items.iter().collect();
    println!("Result: {}", result);
}

fn parse_diagram(diagram_lines: &mut Vec<&str>) -> Vec<Stack> {
    let stack_numbers = diagram_lines.pop().unwrap();
    let stack_numbers = stack_numbers.split(" ");
    let stack_count: usize = stack_numbers.last().unwrap().parse().unwrap();

    diagram_lines.reverse();
    println!("Diagram:");
    for line in diagram_lines.iter() {
        println!("{}", line);
    }

    let mut stacks: Vec<Stack> = Vec::new();
    stacks.resize(stack_count, Stack::new());

    for line in diagram_lines.iter() {
        let line_stack_count = line.len() / 4;
        for i in 0..line_stack_count+1 {
            let crate_name = line.chars().nth(i * 4 + 1).unwrap();
            if crate_name != ' ' {
                stacks[i].push(crate_name);
            }
        }
    }

    println!("Parsed stacks:");
    for stack in &stacks {
        println!("{:?}", stack);
    }

    return stacks;
}
