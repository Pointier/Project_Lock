use core::panic;
use std::clone;

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

pub struct Item {
    core: ItemCore,
    modifiers: Vec<StatModifier>,
}

fn meter_str_to_float(dist: &str) -> f64 {
    dist[..dist.len() - 1]
        .parse::<f64>()
        .expect("Error converting the distance")
}
// TODO: Item builder for implementing common modifiers function
struct ItemBuilder<'a> {
    name: &'a str,
    parsed: &'a ParsedItem,
    modifiers: Vec<StatModifier>,
}

impl<'a> ItemBuilder<'a> {
    fn new(name: &'a str, items: &'a ItemMap) -> Self {
        let parsed = items
            .get(name)
            .unwrap_or_else(|| panic!("{} not found in Item map", name));

        ItemBuilder {
            name,
            parsed,
            modifiers: Vec::new(),
        }
    }

    fn add_modifier(mut self, modifier: StatModifier) -> Self {
        self.modifiers.push(modifier);
        self
    }

    fn with_melee_resist(self) -> Self {
        let val = self
            .parsed
            .expect_prop(self.parsed.melee_resist, "Melee Resist", self.name);
        self.add_modifier(StatModifier {
            stat: StatType::MeleeRes,
            val: ValueType::Int(val),
            cond: Condition::Always,
        })
    }

    fn with_close_range_weapon_damage(self) -> Self {
        let val = self.parsed.expect_prop(
            self.parsed.close_range_bonus_weapon_power,
            "Close range weapon damage bonus",
            self.name,
        );
        let dist = meter_str_to_float(self.parsed.expect_prop(
            self.parsed.close_range_bonus_damage_range.as_deref(),
            "Close range bonus damage distance",
            self.name,
        ));
        self.add_modifier(StatModifier {
            stat: StatType::WepDamage,
            val: ValueType::Int(val),
            cond: Condition::DistanceLessThan(dist),
        })
    }

    fn build(self) -> Item {
        Item {
            core: ItemCore {
                name: self
                    .parsed
                    .expect_prop(self.parsed.name.clone(), "Name", self.name),
                desc: self.parsed.expect_prop(
                    self.parsed.description.clone(),
                    "Description",
                    self.name,
                ),
                cost: self.parsed.expect_prop(self.parsed.cost, "Cost", self.name),
                tier: self.parsed.expect_prop(self.parsed.tier, "Tier", self.name),
                activation: self.parsed.expect_prop(
                    self.parsed.activation.clone(),
                    "Activation",
                    self.name,
                ),
                slot: self
                    .parsed
                    .expect_prop(self.parsed.slot.clone(), "Slot", self.name),
            },
            modifiers: self.modifiers,
        }
    }
}
// Weapon Category
pub fn build_close_quarters(items: &ItemMap) -> Item {
    ItemBuilder::new("Close Quarters", items)
        .with_melee_resist()
        .with_close_range_weapon_damage()
        .build()
}

// Durability Category
// Spirit Category
