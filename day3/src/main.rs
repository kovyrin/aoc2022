use std::str::Lines;
use std::fs::read_to_string;

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
  let mut misplaced_items: Vec<char> = Vec::new();

  loop {
    let bags = [
      lines.next(),
      lines.next(),
      lines.next(),
    ];

    // Check for end of input
    if bags[0].is_none() || bags[1].is_none() || bags[2].is_none() {
      break;
    }

    // Check for items that exist in all 3 bags
    let mut common_item: Option<char> = None;
    for (_i, c1) in bags[0].unwrap().chars().enumerate() {
      for (_j, c2) in bags[1].unwrap().chars().enumerate() {
        for (_k, c3) in bags[2].unwrap().chars().enumerate() {
          if c1 == c2 && c2 == c3 {
            common_item = Some(c1);
          }
        }
      }
    }

    // Add the common item to the misplaced items list
    if common_item.is_none() {
      panic!("Found no common item in bags: {}, {}, {}", bags[0].unwrap(), bags[1].unwrap(), bags[2].unwrap());
    }

    misplaced_items.push(common_item.unwrap());
  }

  let mut priorities_sum: u32 = 0;
  for item in misplaced_items {
    let item_priority = calculate_item_priority(item);
    println!("{}: {}", item, item_priority);
    priorities_sum += item_priority;
  }
  println!("Sum of priorities: {}", priorities_sum);
}

// a-z priorities are 1-26
// A-Z priorities are 27-52
fn calculate_item_priority(item: char) -> u32 {
  if item.is_uppercase() {
    return item as u32 - 38;
  } else {
    return item as u32 - 96;
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn item_priority() {
      assert_eq!(super::calculate_item_priority('a'), 1);
      assert_eq!(super::calculate_item_priority('z'), 26);
      assert_eq!(super::calculate_item_priority('A'), 27);
      assert_eq!(super::calculate_item_priority('Z'), 52);
    }
}
