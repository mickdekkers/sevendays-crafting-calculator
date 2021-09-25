use anyhow::{anyhow, Context};
use chrono::Duration;
use std::fs;

pub mod game_data;

pub fn do_the_thing() -> Result<(), anyhow::Error> {
    let recipes_file =
        fs::File::open("data/config/recipes.xml").context("Failed to open recipes file")?;
    let recipes: game_data::recipe::RecipesFile = serde_xml_rs::from_reader(recipes_file)?;
    // println!("{:#?}", recipes);

    let foo = recipes
        .recipes
        .iter()
        // .filter(|r| r.craft_time > Duration::seconds(0))
        .map(|r| (&r.result.name, r.result.display_name(), &r.craft_time))
        .collect::<Vec<_>>();
    println!("{:#?}", foo);
    Ok(())
}

// pub fn do_the_thing_2() -> Result<(), anyhow::Error> {
//     let file = fs::File::open("data/config/items.xml").context("Failed to open items file")?;
//     let items: game_data::item::ItemsFile = serde_xml_rs::from_reader(file)?;
//     println!("{:#?}", items);

//     Ok(())
// }
