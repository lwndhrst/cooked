use std::collections::hash_map::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct CookbookEntry {
    name: String,
    ingredients: Vec<(String, u8)>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Cookbook {
    recipes: HashMap<String, CookbookEntry>,
}

fn main() {
    let cookbook: Cookbook = ron::from_str(include_str!("../cookbook.ron")).unwrap();

    println!("Cookbook: {}", ron::ser::to_string_pretty(&cookbook, ron::ser::PrettyConfig::default()).unwrap());
}
