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
  for product in products {
    println!("- id={}, name={}, description={}, vendor={}, color={}", product.id, product.name, product.description, product.vendor, product.color);
  }
}
