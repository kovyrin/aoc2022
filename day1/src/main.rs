use std::str::Lines;
use std::fs::read_to_string;

const INPUT_FILE: &str = "real-input.txt";

fn main() {
    let input: String = read_to_string(INPUT_FILE).unwrap();
    let mut lines: Lines = input.lines();
    let mut elves_calories: Vec<u32> = Vec::new();

    let mut current_elf_calories: u32 = 0;
    while let Some(line) = lines.next() {
      if line.is_empty() {
        elves_calories.push(current_elf_calories);
        current_elf_calories = 0;
        continue;
      }
      let calories: u32 = line.parse().unwrap();
      current_elf_calories += calories;
    }
    elves_calories.push(current_elf_calories);

    // find elf with max calories
    let mut max_calories: u32 = 0;
    let mut max_calories_elf: usize = 0;
    for (i, calories) in elves_calories.iter().enumerate() {
      if *calories > max_calories {
        max_calories = *calories;
        max_calories_elf = i;
      }
    }

    println!("Elf {} has the most calories: {}", max_calories_elf+1, max_calories);

    // find the total of top 3 elves
    let mut top_elves_calories: Vec<u32> = elves_calories.clone();
    top_elves_calories.sort();
    top_elves_calories.reverse();
    let top_elves_calories: u32 = top_elves_calories[0..3].iter().sum();
    println!("The top 3 elves have a total of {} calories", top_elves_calories);
}
