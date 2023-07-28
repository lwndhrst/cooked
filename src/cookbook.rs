use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Deserialize, Serialize)]
pub struct Cookbook {
    pub materials: HashMap<Material, String>,
    pub recipes: HashMap<Material, Recipe>,
}

impl Cookbook {
    pub fn required_mats(&self, recipe: Material, n: u64) -> HashMap<Material, u64> {
        let mut result = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((recipe, n));

        while let Some((material, n)) = queue.pop_front() {
            *result.entry(material.to_owned()).or_insert(0) += n;

            if let Some(recipe) = self.recipes.get(&material) {
                for (material, amount) in recipe.ingredients.iter() {
                    queue.push_back((material.to_owned(), amount * n * 10 / 25));
                }
            }
        }

        result
    }
}

#[derive(clap::ValueEnum, Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub enum Material {
    ValenciaMeal,
    Couscous,
    TeffSandwich,
    KingOfJungleHamburg,
    FigPie,
    DatePalmWine,
    FreekehSnakeStew,
    TeffFlourDough,
    Nutmeg,
    Paprika,
    GrilledScorpion,
    TeffBread,
    RedSauce,
    LionMeat,
    PickledVegetables,
    Fig,
    WheatDough,
    Sugar,
    OliveOil,
    DatePalm,
    EssenceOfLiquor,
    LeaveningAgent,
    SnakeMeat,
    Freekeh,
    StarAnise,
    MineralWater,
    TeffFlour,
    Teff,
    ScorpionMeat,
    Butter,
    HighQualityHotPepper,
    Cream,
    Milk,
    Salt,
    Vinegar,
    Wheat,
    Grape,
    WheatFlour,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipe {
    pub ingredients: HashMap<Material, u64>,
}
