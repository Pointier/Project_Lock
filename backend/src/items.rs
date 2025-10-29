use core::panic;

use crate::parser::{ItemMap, ParsedItem, ParsedItemProp};
struct ItemCore {
    name: String,
    desc: String,
    cost: i32,
    tier: i32,
    activation: String,
    slot: String,
}

enum StatType {
    MeleeRes,
    WepDamage,
}
enum ValueType {
    Int(i32),
    Float(f64),
}
enum Condition {
    Always,
    DistanceLessThan(f64),
}
struct StatModifier {
    stat: StatType,
    val: ValueType,
    cond: Condition,
}

struct Item {
    core: ItemCore,
    modifiers: Vec<StatModifier>,
}

impl Item {
    fn new(name: &str, parsed: &ParsedItem, modifiers: Vec<StatModifier>) -> Item {
        Item {
            core: ItemCore {
                name: parsed.expect_prop(parsed.name.clone(), "Name", name),                
                desc: parsed.expect_prop(parsed.description.clone(), "Description", name),
                cost: parsed.expect_prop(parsed.cost, "Cost", name),
                tier: parsed.expect_prop(parsed.tier, "Tier", name),
                activation: parsed.expect_prop(parsed.activation.clone(), "Activation", name),
                slot: parsed.expect_prop(parsed.slot.clone(), "Slot", name),
            },
            modifiers,
        }
    }
}

fn meter_str_to_float(dist: &str) -> f64 {
    dist[..dist.len() - 1]
        .parse::<f64>()
        .expect("Error converting the distance")
}
// TODO: Item builder for implementing common modifiers function
struct ItemBuilder{
    name: &str,
    parsed:&ParsedItem,
    modifiers: Vec<StatModifier>,
}
// Weapon Category
fn build_close_quarters(items: &ItemMap) -> Item {
    let name = "Close Quarters";
    let item = items
        .get(name)
        .unwrap_or_else(|| panic!("{} is missing", name));

    let melee_res = StatModifier {
        stat: StatType::MeleeRes,
        val: ValueType::Int(
            item.melee_resist
                .unwrap_or_else(|| panic!("Melee resist missing for {}", name)),
        ),
        cond: Condition::Always,
    };

    let dist = meter_str_to_float(
        &item
            .close_range_bonus_damage_range
            .clone()
            .unwrap_or_else(|| panic!("Range missing for {}", name)),
    );
    let weapon_damage = item
        .close_range_bonus_weapon_power
        .unwrap_or_else(|| panic!("Close range weapon damage missing for {}", name));
    let close_weap_damage = StatModifier {
        stat: StatType::WepDamage,
        val: ValueType::Int(weapon_damage),
        cond: Condition::DistanceLessThan(dist),
    };
    let modifiers = vec![melee_res, close_weap_damage];

    Item::new(name, item, modifiers)
}

// Durability Category
// Spirit Category
