pub struct Weapon {
    base_damage: f64,
    boon_damage: f64,
    fire_rate: f64,
    reload_time: f64,
    mag_size: i32,
}

impl Weapon {
    pub fn new(
        base_damage: f64,
        boon_damage: f64,
        fire_rate: f64,
        reload_time: f64,
        mag_size: i32,
    ) -> Weapon {
        if base_damage <= 0.0
            || boon_damage <= 0.0
            || fire_rate <= 0.0
            || reload_time <= 0.0
            || mag_size <= 0
        {
            panic!(
                "Negative value for a base stat of the weapon, base damage {}, boon damage {}, fire rate {}, reload time {}, mag size {}",
                base_damage, boon_damage, fire_rate, reload_time, mag_size
            )
        }
        Weapon {
            base_damage,
            boon_damage,
            fire_rate,
            reload_time,
            mag_size,
        }
    }

    pub fn current_damage(&self, boon_level: i32) -> f64 {
        self.base_damage + boon_level as f64 * self.boon_damage
    }
}

fn current_fire_rate(base_fire_rate: f64, bonus_fire_rate: i32) -> f64 {
    let percentage_fire_rate: f64 = 1.0 + bonus_fire_rate as f64 / 100.0;
    base_fire_rate * percentage_fire_rate
}

fn dps_no_reload(damage: f64, fire_rate: f64) -> f64 {
    damage * fire_rate
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn weapon_new() {
        let weapon = Weapon::new(10.0, 0.5, 2.5, 2.0, 15);

        assert_relative_eq!(weapon.base_damage, 10.0);
        assert_relative_eq!(weapon.boon_damage, 0.5);
        assert_relative_eq!(weapon.fire_rate, 2.5);
        assert_relative_eq!(weapon.reload_time, 2.0);
        assert_eq!(weapon.mag_size, 15);
    }

    #[test]
    #[should_panic]
    fn weapon_new_panic_negative_value() {
        let weapon = Weapon::new(-5.0, 0.0, 3.0, 4.0, 20);
    }

    #[test]
    fn weapon_current_fire_rate() {
        let weapon = Weapon::new(10.0, 0.5, 2.5, 2.0, 15);
        let boon_level = 5;
        assert_relative_eq!(weapon.current_damage(boon_level), 12.5);
    }

    #[test]
    fn test_dps_no_reload() {
        assert_relative_eq!(dps_no_reload(5.0, 3.0), 15.0);
    }

    #[test]
    fn test_current_fire_rate() {
        assert_relative_eq!(current_fire_rate(2.0, 50), 3.0)
    }

    #[test]
    fn negative_fire_rate() {
        assert_relative_eq!(current_fire_rate(10.0, -40), 6.0)
    }
}
