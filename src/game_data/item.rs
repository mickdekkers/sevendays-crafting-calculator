use chrono::Duration;
use itertools::Itertools;
use serde::Deserialize;
use serde_with::{serde_as, CommaSeparator, DisplayFromStr, DurationSeconds, StringWithSeparator};

// <item name="drinkCanEmpty">
// 	<property name="HoldType" value="14"/>
// 	<property name="Meshfile" value="#Other/Items?Food/can_emptyPrefab.prefab"/>
// 	<property name="DropMeshfile" value="#Other/Items?Misc/sack_droppedPrefab.prefab"/>
// 	<property name="Material" value="Mmetal"/>
// 	<property name="Stacknumber" value="500"/>
// 	<property name="EconomicValue" value="5"/>
// 	<property name="CraftingIngredientTime" value="9"/>
// 	<property name="Group" value="Resources"/>
// </item>

// TODO: not sure properties can be parsed with serde_xml_rs 🤔
#[derive(Debug, Deserialize)]
pub struct Property {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(default)]
    pub name: Option<String>,

    #[serde(rename = "property", default)]
    pub properties: Vec<Property>,
}

#[derive(Debug, Deserialize)]
pub struct ItemsFile {
    #[serde(rename = "$value")]
    pub items: Vec<Item>,
}
