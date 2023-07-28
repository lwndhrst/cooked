use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Cookbook {
    pub recipes: HashMap<CookbookKey, CookbookValue>,
}

#[derive(clap::ValueEnum, Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub enum CookbookKey {
    ValenciaMeal,
    Couscous,
    TeffSandwich,
    KingOfJungleHamburg,
    FigPie,
    DatePalmWine,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CookbookValue {
    pub name: String,
    pub ingredients: HashMap<CookbookKey, u8>,
}
