use std::collections::HashMap;

use serde::{Deserialize, Serialize};

const DATA_FILE: &str = "products.json";

#[derive(Serialize, Deserialize)]
struct Product {
  id: i32,
  name: String,
  description: String,
  vendor: String,
  color: String,
}

// build a word index from a vector of Product structs
fn build_word_index(products: &[Product]) -> HashMap<String, Vec<&Product>> {
  let mut word_index: HashMap<String, Vec<&Product>> = HashMap::new();

  for product in products {
    let words: Vec<&str> = product.name.split(" ").collect();
    for word in words {
      let keyword = word.to_string().to_lowercase();
      if !word_index.contains_key(&keyword) {
        word_index.insert(keyword.clone(), Vec::new());
      }
      let keyword_products = word_index.get_mut(&keyword).unwrap();
      keyword_products.push(product);
    }
  }

  return word_index;
}

// load json file DATA_FILE
// return a vector of Product structs
fn load_products(filename: &str) -> Vec<Product> {
  let data = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
  return serde_json::from_str(&data).unwrap();
}

fn main() {
  // load the data file
  let products = load_products(DATA_FILE);

  println!("Products: ");
  for product in &products {
    println!("- id={}, name={}, description={}, vendor={}, color={}", product.id, product.name, product.description, product.vendor, product.color);
  }

  let word_index = build_word_index(products.as_slice());

  let keyboards = find_by_keyword(&word_index, "keyboard");
  print_results("Found products with keyword 'keyboard':", &keyboards);

  let bananas = find_by_keyword(&word_index, "banana");
  print_results("Found products with keyword 'banana':", &bananas);
}

fn print_results(title: &str, products: &Vec<&Product>) {
  println!("\n{}", title);
  if products.is_empty() {
    println!("No results found");
    return;
  }

  for product in products {
    println!("- id={}, name={}, description={}, vendor={}, color={}", product.id, product.name, product.description, product.vendor, product.color);
  }
}

fn find_by_keyword<'a>(word_index: &'a HashMap<String, Vec<&Product>>, keyword: &str) -> Vec<&'a Product> {
  let keyword = keyword.to_string().to_lowercase();
  if !word_index.contains_key(&keyword) {
    return Vec::new();
  }
  return word_index.get(&keyword).unwrap().clone();
}
