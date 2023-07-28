mod cookbook;

use clap::Parser;
use cookbook::{Cookbook, Material};

#[derive(Debug, Parser)]
struct Args {
    /// Which recipe do you want to cook?
    #[clap(value_enum)]
    recipe: Material,

    /// How many do you need?
    #[arg(short)]
    n: u64,

    /// What's your additional max proc chance from mastery? (in %)
    #[arg(short, default_value_t = 10)]
    m: u64,
}

fn main() {
    let cookbook: Cookbook = ron::from_str(include_str!("../cookbook.ron")).unwrap();
    let args = Args::parse();

    let materials = cookbook.required_mats(args.recipe, args.n, args.m);
    for (material, amount) in materials {
        println!("{}: {}", cookbook.materials.get(&material).unwrap(), amount);
    }
}
