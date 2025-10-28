use core::panic;

use crate::parser::{ItemMap, ParsedItem};
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
    DistanceLessThan,
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
    fn new(name: &str, parsed:&ParsedItem, modifiers: Vec<StatModifier>) -> Item {
        Item {
            core: ItemCore {
                name: parsed
                    .name
                    .clone()
                    .unwrap_or_else(|| panic!("Item name is missing {}", name)),
                desc: parsed
                    .description
                    .clone()
                    .unwrap_or_else(|| panic!("Description is missing {}", name)),
                cost: parsed
                    .cost
                    .unwrap_or_else(|| panic!("Cost is missing {}", name)),
                tier: parsed
                    .tier
                    .unwrap_or_else(|| panic!("Tier is missing {}", name)),
                activation: parsed
                    .activation
                    .clone()
                    .unwrap_or_else(|| panic!("Activation is missing {}", name)),
                slot: parsed
                    .slot
                    .clone()
                    .unwrap_or_else(|| panic!("Slot is missing {}", name)),
            },
            modifiers,
        }
    }
}

// Weapon Category
fn build_close_quarters(items: &ItemMap) -> Item {
    let name = "Close Quarters";
    let item = items
        .get(name)
        .unwrap_or_else(|| panic!("{} is missing", name));
    let modifiers = vec![StatModifier {
        stat: StatType::MeleeRes,
        val: ValueType::Int(
            item.melee_resist
                .unwrap_or_else(|| panic!("Melee resist missing for {}", name)),
        ),
        cond: Condition::Always,
    }];
    Item::new(name, item, modifiers)
}

// Durability Category
// Spirit Category
