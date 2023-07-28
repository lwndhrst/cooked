mod cookbook;

use clap::Parser;
use cookbook::*;

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
