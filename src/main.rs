mod cookbook;

use clap::Parser;
use cookbook::{Cookbook, Material};

#[derive(Debug, Parser)]
struct Args {
    /// The recipe you wish to cook
    #[clap(value_enum)]
    recipe: Material,

    /// Number of cooks
    #[arg(short)]
    n: u64,
}

fn main() {
    let cookbook: Cookbook = ron::from_str(include_str!("../cookbook.ron")).unwrap();
    let args = Args::parse();

    let materials = cookbook.required_mats(args.recipe, args.n);
    for (material, amount) in materials {
        println!("{}: {}", cookbook.materials.get(&material).unwrap(), amount);
    }
}
