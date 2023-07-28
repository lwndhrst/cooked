use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct Cookbook {
    recipes: HashMap<CookbookKey, CookbookValue>,
}

#[derive(clap::ValueEnum, Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
enum CookbookKey {
    ValenciaMeal,
    Couscous,
    TeffSandwich,
    KingOfJungleHamburg,
    FigPie,
    DatePalmWine,
}

#[derive(Debug, Deserialize, Serialize)]
struct CookbookValue {
    name: String,
    ingredients: HashMap<CookbookKey, u8>,
}

#[derive(Debug, Parser)]
struct Args {
    /// The recipe you wish to cook
    #[clap(value_enum)]
    recipe: CookbookKey,

    /// Number of cooks
    #[arg(short)]
    n: u64,
}

fn main() {
    let cookbook: Cookbook = ron::from_str(include_str!("../cookbook.ron")).unwrap();

    let args = Args::parse();

    println!("Selected: {:?}", &args.recipe);
    println!("Cookbook: {:?}", cookbook.recipes[&args.recipe]);
}
