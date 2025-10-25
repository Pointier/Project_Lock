//TODO: add weap/dur/spirit threshold as hard coded value here ?
use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    name: Option<String>,
    description: Option<String>,
    cost: Option<i32>,
    slot: Option<String>,
    bonus_fire_rate: Option<i32>,
}

pub type ItemMap = HashMap<String, Item>;

fn parse_items(json: &str) -> ItemMap {
     serde_json::from_str(json).expect("Cant read data")
}
mod weapon;
fn main() {
    let path_data = "/home/demonz/programmation/Project_Lock/data/item-data.json";
    let item_data = fs::read_to_string(path_data).expect("Failed to read file");
    let items: ItemMap = serde_json::from_str(&item_data).expect("Cant read data");
    println!(
        "{:#?}",
        items.get("upgrade_ability_refresher").unwrap().name
    );
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn data_parsed() {
        let data = r#"
        {
        "upgrade_ricochet": {
                "Name": "Ricochet",
                "Description": "Your bullets will <span class=\"highlight\">ricochet</span> on enemies near your target, <span class=\"highlight\">applying any bullet procs</span> and <span class=\"highlight\">dealing a percentage of the original damage.</span>",
                "Cost": 6400,
                "Tier": 4,
                "Activation": "Passive",
                "Slot": "Weapon",
                "Components": null,
                "TargetTypes": [
                    "HeroEnemy",
                    "BossEnemy",
                    "TrooperEnemy",
                    "PropEnemy",
                    "MinionEnemy",
                    "Neutral"
                ],
                "ShopFilters": [
                    "ClipSize"
                ],
                "IsDisabled": false,
                "AbilityUnitTargetLimit": 1,
                "ChannelMoveSpeed": -1,
                "RicochetDamagePercent": 65,
                "RicochetRadius": "13m",
                "RicochetTargetsTooltipOnly": 2,
                "BonusFireRate": 20
            }
        }"#;
        let items: ItemMap = parse_items(data);
        let item = items.get("upgrade_ricochet").unwrap();
        assert_eq!(item.name.as_ref().unwrap(), "Ricochet");
        assert_eq!(
            item.description.as_ref().unwrap(),
            "Your bullets will <span class=\"highlight\">ricochet</span> on enemies near your target, <span class=\"highlight\">applying any bullet procs</span> and <span class=\"highlight\">dealing a percentage of the original damage.</span>"
        );
        assert_eq!(item.cost.unwrap(), 6400);
        assert_eq!(item.slot.as_ref().unwrap(),"Weapon");
        assert_eq!(item.bonus_fire_rate.unwrap(), 20)
    }
}
