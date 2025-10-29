//TODO: add weap/dur/spirit threshold as hard coded value here ?
use std::{ fs};

mod parser;
use parser::*;
mod items;
mod weapon;


fn main() {
    let path_data = "/home/demonz/programmation/Project_Lock/data/item-data.json";
    let item_data = fs::read_to_string(path_data).expect("Failed to read file");
    let items: ItemMap = parse_items(&item_data);
    valid_items(items);
}

