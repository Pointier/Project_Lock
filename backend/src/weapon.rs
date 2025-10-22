//TODO: add enum for type of weapon or way to handle it
#[derive(Debug, PartialEq)]
pub enum WeaponType {
    Auto,
    Burst { burst_time: f64 }, // In seconds
    Shotgun { base_pellet: f64, pellet_boon: f64 },
}
pub struct Weapon {
    base_damage: f64,
    boon_damage: f64,
    fire_rate: f64,   // Bullets per second
    reload_time: f64, // In seconds
    mag_size: i32,
    weapon_type: WeaponType,
    min_falloff: f64, // In meter
    max_falloff: f64, // In meter
}

pub struct WeaponState {
    pub boon_level: i32,
    pub bonus_damage: i32,
    pub bonus_fire_rate: i32,
    pub bonus_ammo: i32,
    pub bonus_falloff: i32,
}

impl WeaponState {
    pub fn default() -> WeaponState {
        WeaponState {
            boon_level: 0,
            bonus_damage: 0,
            bonus_fire_rate: 0,
            bonus_ammo: 0,
            bonus_falloff: 0,
        }
    }
    pub fn new(
        boon_level: i32,
        bonus_damage: i32,
        bonus_fire_rate: i32,
        bonus_ammo: i32,
        bonus_falloff: i32,
    ) -> WeaponState {
        if boon_level < 0 {
            panic!("Negative value for boon level {}", boon_level)
        }
        WeaponState {
            boon_level,
            bonus_damage,
            bonus_fire_rate,
            bonus_ammo,
            bonus_falloff,
        }
    }
}

impl Weapon {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        base_damage: f64,
        boon_damage: f64,
        fire_rate: f64,
        reload_time: f64,
        mag_size: i32,
        weapon_type: WeaponType,
        start_falloff: f64,
        end_falloff: f64,
    ) -> Weapon {
        if base_damage <= 0.0
            || boon_damage <= 0.0
            || fire_rate <= 0.0
            || reload_time <= 0.0
            || mag_size <= 0
            || start_falloff <= 0.0
            || end_falloff <= 0.0
        {
            panic!(
                "Negative value for a base stat of the weapon, base damage {}, boon damage {}, fire rate {}, reload time {}, mag size {}, start_falloff {}, end_falloff {}",
                base_damage,
                boon_damage,
                fire_rate,
                reload_time,
                mag_size,
                start_falloff,
                end_falloff
            )
        }
        Weapon {
            base_damage,
            boon_damage,
            fire_rate,
            reload_time,
            mag_size,
            weapon_type,
            min_falloff: start_falloff,
            max_falloff: end_falloff,
        }
    }

    /*TODO: missing parameter for weapon damage :
    - Flat Weapon Damage (coming only from abilities right now), is not multiplied by weapon damage
    - bullet resistance and reduction
    - increased bullet damage (debuff inflicted directly by the heroes)
    - crit multiplier
    - damage per weapon type (burst, auto, shotgun)
     */
    pub fn current_damage(&self, state: &WeaponState) -> f64 {
        let percentage_damage: f64 = 1.0 + state.bonus_damage as f64 / 100.0;
        (self.base_damage + state.boon_level as f64 * self.boon_damage) * percentage_damage
    }

    pub fn current_fire_rate(&self, state: &WeaponState) -> f64 {
        let percentage_fire_rate: f64 = 1.0 + state.bonus_fire_rate as f64 / 100.0;
        self.fire_rate * percentage_fire_rate
    }
    pub fn dps_no_reload(&self, state: &WeaponState) -> f64 {
        let percent_damage = 1.0 + state.bonus_damage as f64 / 100.0;
        let percent_fire_rate = 1.0 + state.bonus_fire_rate as f64 / 100.0;
        (self.base_damage + self.boon_damage * state.boon_level as f64)
            * percent_damage
            * self.fire_rate
            *percent_fire_rate
    }
    pub fn falloff_damage(&self, dist: f64, state: &WeaponState) -> f64 {
        //Linear Interpolation:
        let min_falloff = self.min_falloff * (1.0 + state.bonus_falloff as f64 / 100.0);

        let max_falloff = self.max_falloff * (1.0 + state.bonus_falloff as f64 / 100.0);
        let min_mult = 0.1;
        let max_mult = 1.0;
        if dist < min_falloff {
            max_mult
        } else if dist > max_falloff {
            min_mult
        } else {
            let range = max_mult - min_mult;
            let max_damage = 1.0;
            let factor = (dist - min_falloff) / (max_falloff - min_falloff);
            max_damage - range * factor
        }
        // damage range between no falloff and max falloff
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn weapon_new() {
        let weapon = Weapon::new(10.0, 0.5, 2.5, 2.0, 15, WeaponType::Auto, 12.0, 24.0);

        assert_relative_eq!(weapon.base_damage, 10.0);
        assert_relative_eq!(weapon.boon_damage, 0.5);
        assert_relative_eq!(weapon.fire_rate, 2.5);
        assert_relative_eq!(weapon.reload_time, 2.0);
        assert_eq!(weapon.mag_size, 15);
        assert_eq!(weapon.weapon_type, WeaponType::Auto);
        assert_relative_eq!(weapon.min_falloff, 12.0);
        assert_relative_eq!(weapon.max_falloff, 24.0);
    }

    #[test]
    #[should_panic]
    fn weapon_new_panic_negative_value() {
        let _weapon = Weapon::new(-5.0, 0.0, 3.0, 4.0, 20, WeaponType::Auto, 12.0, 24.0);
    }

    #[test]
    fn weapon_state_default() {
        let state = WeaponState::default();
        assert_eq!(state.boon_level, 0);

        assert_eq!(state.bonus_damage, 0);
        assert_eq!(state.bonus_fire_rate, 0);
        assert_eq!(state.bonus_ammo, 0);
    }
    #[test]
    fn weapon_state_new() {
        let state = WeaponState::new(5, 50, 100, 0, 0);
        assert_eq!(state.boon_level, 5);
        assert_eq!(state.bonus_damage, 50);
        assert_eq!(state.bonus_fire_rate, 100);
        assert_eq!(state.bonus_ammo, 0);
        assert_eq!(state.bonus_falloff, 0)
    }

    #[test]
    #[should_panic]
    fn weapon_state_new_boon_panic() {
        let _state = WeaponState::new(-1, 10, 10, 10, 0);
    }

    #[test]
    fn weapon_default_current_damage() {
        let weapon = Weapon::new(10.0, 0.5, 2.5, 2.0, 15, WeaponType::Auto, 12.0, 24.0);
        let state = WeaponState::default();
        assert_relative_eq!(weapon.current_damage(&state), 10.0);
    }

    #[test]
    fn weapon_falloff_damage() {
        let weapon = Weapon::new(10.0, 0.5, 2.5, 2.0, 15, WeaponType::Auto, 12.0, 24.0);
        let state = WeaponState::default();
        assert_relative_eq!(weapon.falloff_damage(16.0, &state), 0.7);
    }

    #[test]
    fn weapon_min_falloff() {
        let weapon = Weapon::new(10.0, 0.5, 2.5, 2.0, 15, WeaponType::Auto, 12.0, 24.0);
        let state = WeaponState::default();
        assert_relative_eq!(weapon.falloff_damage(5.0, &state), 1.0);
    }

    #[test]
    fn weapon_max_falloff() {
        let weapon = Weapon::new(10.0, 0.5, 2.5, 2.0, 15, WeaponType::Auto, 12.0, 24.0);
        let state = WeaponState::default();
        assert_relative_eq!(weapon.falloff_damage(30.0, &state), 0.1)
    }
    #[test]
    fn weapon_current_damage_bonus_damage() {
        let weapon = Weapon::new(10.0, 0.5, 2.5, 2.0, 15, WeaponType::Auto, 12.0, 24.0);
        let state = WeaponState {
            bonus_damage: 50,
            ..WeaponState::default()
        };
        assert_relative_eq!(weapon.current_damage(&state), 15.0)
    }

    #[test]
    fn weapon_current_damage_boon_level() {
        let weapon = Weapon::new(10.0, 0.5, 2.5, 2.0, 15, WeaponType::Auto, 12.0, 24.0);
        let state = WeaponState {
            boon_level: 5,
            ..WeaponState::default()
        };
        assert_relative_eq!(weapon.current_damage(&state), 12.5)
    }

    #[test]
    fn weapon_current_fire_rate() {
        let weapon = Weapon::new(10.0, 0.5, 2.5, 2.0, 15, WeaponType::Auto, 12.0, 24.0);
        let state = WeaponState {
            bonus_fire_rate: 100,
            ..WeaponState::default()
        };
        assert_relative_eq!(weapon.current_fire_rate(&state), 5.0);
    }

    #[test]
    fn weapon_negative_fire_rate() {
        let weapon = Weapon::new(10.0, 0.5, 2.5, 2.0, 15, WeaponType::Auto, 12.0, 24.0);
        let state = WeaponState {
            bonus_fire_rate: -40,
            ..WeaponState::default()
        };
        assert_relative_eq!(weapon.current_fire_rate(&state), 1.5)
    }

    #[test]
    fn weapon_dps_no_reload() {
        let weapon = Weapon::new(10.0, 0.5, 2.5, 2.0, 15, WeaponType::Auto, 12.0, 24.0);
        let state = WeaponState {
            bonus_damage: 100,
            bonus_fire_rate: 100,
            boon_level: 10,
            ..WeaponState::default()
        };
        assert_relative_eq!(weapon.dps_no_reload(&state), 150.0)
    }
}
