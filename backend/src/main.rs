//TODO: add weap/dur/spirit threshold as hard coded value here ?
use std::{collections::HashMap, fs};
use serde::Deserialize;
use serde_json;

#[derive(Debug,Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item{
    name:Option<String>,
    description:Option<String>,
    cost:Option<i32>,
}

pub type ItemMap = HashMap<String, Item>;
mod weapon;
fn main() {
    let item_data = fs::read_to_string("/home/demonz/programmation/Project_Lock/data/item-data.json").expect("Failed to read file");
    let items :ItemMap = serde_json::from_str(&item_data).expect("Cant read data");


    println!("{:#?}", items.get("upgrade_ability_refresher").unwrap().name);

}
