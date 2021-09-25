use anyhow::{anyhow, Context};
use chrono::Duration;
use std::fs;

pub mod game_data;

pub fn do_the_thing() -> Result<(), anyhow::Error> {
    // let path = {
    //     let mut p = std::env::current_dir()?;
    //     p.push("data/config/recipes.xml");
    //     p
    // };
    // print!("{:?}", path);
    let file = fs::File::open("data/config/recipes.xml").context("Failed to open recipes file")?;
    let recipes: game_data::recipe::RecipesFile = serde_xml_rs::from_reader(file)?;
    // println!("{:#?}", recipes);

    // let all_tags: Vec<&String> = recipes.get_unique_recipe_tags();
    // println!("{:#?}", all_tags);

    // let all_craft_tools: Vec<&String> = recipes.get_unique_craft_tools();
    // println!("{:#?}", all_craft_tools);

    // let foo = recipes
    //     .recipes
    //     .iter()
    //     .find(|r| r.result.name == "resourceMilitaryFiber");
    // println!("{:#?}", foo);

    // let forge_category_recipes = recipes
    //     .recipes
    //     .iter()
    //     .filter(|r| r.is_wildcard_forge_category())
    //     .map(|r| &r.result.name)
    //     .collect::<Vec<_>>();
    // println!("{:#?}", forge_category_recipes);

    // let crucible_recipes = recipes
    //     .recipes
    //     .iter()
    //     .filter(|r| matches!(r.craft_tool, Some(game_data::recipe::CraftTool::Crucible)))
    //     .map(|r| &r.result.name)
    //     .collect::<Vec<_>>();
    // println!("{:#?}", crucible_recipes);

    let recipes_with_craft_time = recipes
        .recipes
        .iter()
        .filter(|r| r.craft_time > Duration::seconds(0))
        .map(|r| (&r.result.name, &r.craft_time))
        .collect::<Vec<_>>();
    println!("{:#?}", recipes_with_craft_time);
    Ok(())
}
