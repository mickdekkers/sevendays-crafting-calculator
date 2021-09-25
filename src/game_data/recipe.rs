use crate::game_data::locale;
use chrono::Duration;
use itertools::Itertools;
use serde::Deserialize;
use serde_with::{serde_as, CommaSeparator, DisplayFromStr, DurationSeconds, StringWithSeparator};

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct ItemSpecifier {
    pub name: String,
    #[serde_as(as = "DisplayFromStr")]
    pub count: u32,
}

impl ItemSpecifier {
    pub fn display_name(&self) -> &str {
        locale::DISPLAY_NAMES
            .get(&self.name)
            .map(|s| s.as_str())
            .unwrap_or(&"DISPLAY_NAME_NOT_FOUND")
    }
}

// <recipe name="drinkCanEmpty" count="1" material_based="true" craft_area="forge" tags="perkAdvancedEngineering">
// 	<ingredient name="unit_iron" count="5"/>
// 	<ingredient name="unit_clay" count="1"/>
// 	<effect_group>
// 		<passive_effect name="CraftingIngredientCount" operation="perc_add" level="0,1,2,3,4" value=".25,.25,.25,.177,.12" tags="unit_iron"/>
// 	</effect_group>
// </recipe>

// All tags:
// [
//     "boiledWater",
//     "cementMixerCrafting",
//     "chemStationCrafting",
//     "learnable",
//     "perkAdvancedEngineering",
//     "perkArchery",
//     "perkBoomstick",
//     "perkBrawler",
//     "perkChefBoiledWater",
//     "perkDeadEye",
//     "perkDeepCuts",
//     "perkDemolitionsExpert",
//     "perkElectrocutioner",
//     "perkGreaseMonkey",
//     "perkGunslinger",
//     "perkHeavyArmor",
//     "perkJavelinMaster",
//     "perkLightArmor",
//     "perkLivingOffTheLandCrafting",
//     "perkMachineGunner",
//     "perkMasterChef",
//     "perkMiner69r",
//     "perkPummelPete",
//     "perkSalvageOperations",
//     "perkSkullCrusher",
//     "perkTurrets",
//     "salvageScrap",
//     "workbenchCrafting",
// ]

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CraftArea {
    Campfire,
    CementMixer,
    ChemistryStation,
    Forge,
    Workbench,
}

#[derive(Debug, Deserialize)]
pub enum CraftTool {
    #[serde(rename = "toolBeaker")]
    Beaker,
    #[serde(rename = "toolCookingGrill")]
    CookingGrill,
    #[serde(rename = "toolCookingPot")]
    CookingPot,
    #[serde(rename = "toolForgeCrucible")]
    Crucible,
}

fn zero_duration() -> Duration {
    Duration::seconds(0)
}

// TODO: "material_based" recipes supposedly mean it can be smelted down in the forge... Does that mean the recipe works the other way around for that? Check if "material_based" recipes exist with craft_area other than forge. Update: probably not, see resourceRockSmall
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Recipe {
    #[serde(flatten)]
    pub result: ItemSpecifier,

    #[serde(rename = "ingredient", default)]
    pub ingredients: Vec<ItemSpecifier>,

    pub craft_area: Option<CraftArea>,

    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    #[serde(default)]
    pub tags: Vec<String>,

    // wildcard_forge_category has to do with forge_category in materials.xml
    // wildcard_forge_category is an empty tag if present, so this is deserialized a bit strangely and we have to use the is_wildcard_forge_category function to expose the actual value
    #[serde(default)]
    wildcard_forge_category: Option<String>,

    #[serde(default)]
    pub craft_tool: Option<CraftTool>,

    #[serde_as(as = "DurationSeconds<String>")]
    #[serde(default = "zero_duration")]
    pub craft_time: Duration,
}

impl Recipe {
    pub fn is_wildcard_forge_category(&self) -> bool {
        self.wildcard_forge_category.is_some()
    }
}

#[derive(Debug, Deserialize)]
pub struct RecipesFile {
    #[serde(rename = "$value")]
    pub recipes: Vec<Recipe>,
}

impl RecipesFile {
    /// Returns a list of all recipe tags present in the recipe file
    pub fn get_unique_recipe_tags(&self) -> Vec<&String> {
        self.recipes
            .iter()
            .flat_map(|r| &r.tags)
            .sorted()
            .dedup()
            .collect()
    }
}
