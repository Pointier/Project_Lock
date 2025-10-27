//TODO: add weap/dur/spirit threshold as hard coded value here ?
use serde::Deserialize;
use std::{collections::HashMap, fs};
mod weapon;

// Missing key from the json like for cheat death, idk if needed later
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParsedItem {
    name: Option<String>,
    description: Option<String>,
    cost: Option<i32>,
    tier:Option<i32>,
    activation:Option<String>,
    slot: Option<String>,
    components:Option<Vec<String>>,
    is_disabled:Option<bool>,
    ability_cooldown:Option<f64>,
    bonus_health:Option<i32>,
    bullet_resist:Option<i32>,
    bonus_fire_rate: Option<i32>,
}

fn valid_item(item:ParsedItem)->bool{
    item.name.is_some() && !item.is_disabled.expect("Missing value for is_disabled field (item).")

}
pub type ParsedItemMap = HashMap<String, ParsedItem>;

fn parse_items(json: &str) -> ParsedItemMap {
     serde_json::from_str(json).expect("Cant read item json")
}

fn item_vec(items: ParsedItemMap)->Vec<ParsedItem>{

}
fn main() {
    let path_data = "/home/demonz/programmation/Project_Lock/data/item-data.json";
    let item_data = fs::read_to_string(path_data).expect("Failed to read file");
    let items: ParsedItemMap = serde_json::from_str(&item_data).expect("Cant read data");
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
        let items: ParsedItemMap = parse_items(data);
        let item = items.get("upgrade_ricochet").unwrap();
        assert_eq!(item.name.as_ref().unwrap(), "Ricochet");
        assert_eq!(
            item.description.as_ref().unwrap(),
            "Your bullets will <span class=\"highlight\">ricochet</span> on enemies near your target, <span class=\"highlight\">applying any bullet procs</span> and <span class=\"highlight\">dealing a percentage of the original damage.</span>"
        );
        assert_eq!(item.cost.unwrap(), 6400);
        assert_eq!(item.slot.as_ref().unwrap(),"Weapon");
        assert_eq!(item.bonus_fire_rate.unwrap(), 20);
        assert!(!item.is_disabled.unwrap())
    }

    #[test]
    fn item_is_valid(){
        let data = r#"
        {
            "Name": "Echo Shard",
            "Description": null,
            "Cost": 6400,
            "Tier": 4,
            "Activation": "InstantCast",
            "Slot": "Tech",
            "Components": null,
            "TargetTypes": null,
            "ShopFilters": [
                "Movement",
                "MagicDamage"
            ],
            "IsDisabled": false,
            "AbilityCooldown": 30.0,
            "AbilityUnitTargetLimit": 1,
            "AbilityCastDelay": 0.2,
            "ChannelMoveSpeed": -1,
            "CooldownReduction": 5,
            "BonusFireRate": 5,
            "TechResist": 5,
            "BulletResist": 5
        }
            "#;
        let item :ParsedItem= serde_json::from_str(data).unwrap();
        // item is valid if have a name and IsDisabled is false
        assert!(valid_item(item), "Item wasnt valid");

    }
    #[test]
    fn item_is_not_valid(){
        let data= r#"
        {
            "Name": null,
            "Description": null,
            "Cost": 9999,
            "Tier": 5,
            "Activation": "Passive",
            "Slot": "Tech",
            "Components": null,
            "TargetTypes": null,
            "ShopFilters": null,
            "IsDisabled": false,
            "AbilityUnitTargetLimit": 1,
            "ChannelMoveSpeed": -1
        }
            "#;
        let item :ParsedItem= serde_json::from_str(data).unwrap();
        assert!(!valid_item(item), "Item was valid");
    }
    #[test]
    fn parsed_item_vector() {
        let data  = r#""#;

    }
}
