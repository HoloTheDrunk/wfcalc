use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Duration,
};

use crate::{damage::*, enemy::Faction};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum ModEffect {
    //// Warframe / Companion / Vehicle
    // Ability
    AbilityDuration(f32),
    AbilityEfficiency(f32),
    AbilityStrength(f32),
    AbilityRange(f32),
    // Health
    HealthCapacity(f32),
    HealthRegen(f32),
    // Shield
    ShieldCapacity(f32),
    ShieldRestore(f32),
    ShieldRechargeRate(f32),
    ShieldRechargeDelay(f32),
    ShieldGateDuration(f32),
    // Armor
    Armor(f32),
    // Energy
    Energy(f32),
    EnergyRegen(f32),
    // Radar
    RadarEnemy(f32),
    RadarLoot(f32),
    // Movement
    SprintSpeed(f32),
    Slide(f32),
    Friction(f32),
    // Defense
    DamageReduction(f32),
    DamageRedirection(f32),
    KnockdownResistance(f32),
    PhysicalDamageResistance(f32),
    EnvironmentalIceResistance(f32),
    ElementResistance(Element, f32),
    EnemyAccuracy(f32),

    //// Weapons
    Physical(Ips, f32),
    Elemental(Element, f32),
    Bane(Faction, f32),
    // Status
    StatusChance(f32),
    StatusDuration(f32),
    StatusDamage(f32),
    // Critical
    CriticalChance(f32),
    CriticalDamage(f32),
    // Misc.
    Multishot(f32),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Trigger {
    Damaged,
    HealthOrbPickup,
    EnergyOrbPickup,
    AmmoPickup,
    Hit,
    Kill,
    WeakpointHit,
    WeakpointKill,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum ReduceAmount {
    Flat(u32),
    Relative(f32),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum TimeoutBehaviour {
    Drop,
    Reduce(ReduceAmount),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum StackingBehaviour {
    Timed {
        duration: Duration,
        timeout: TimeoutBehaviour,
        resets_on_stack: bool,
    },
    Forever,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Stacking {
    pub max: u32,
    pub behaviour: StackingBehaviour,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ModStat {
    pub trigger: Option<Trigger>,
    pub stacking: Option<Stacking>,
    pub effect: ModEffect,
}

impl From<ModEffect> for ModStat {
    fn from(effect: ModEffect) -> Self {
        Self {
            trigger: None,
            stacking: None,
            effect,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Mod {
    pub name: String,
    pub stats: Vec<ModStat>,
}

#[derive(Debug)]
pub struct ModLibrary {
    path: PathBuf,
    mods: HashMap<String, Mod>,
}

impl ModLibrary {
    pub fn load(path: &Path) -> Self {
        let file_content = std::fs::read_to_string(path).expect("Should read mod library file");
        let mods = ron::from_str::<Vec<Mod>>(file_content.as_str())
            .expect("Should deserialize mod library");
        Self {
            path: path.to_path_buf(),
            mods: mods.into_iter().map(|m| (m.name.clone(), m)).collect(),
        }
    }

    pub fn get(&self, mod_name: &str) -> Option<&Mod> {
        self.mods.get(mod_name)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_mods() {
        ModLibrary::load(Path::new("data/mods.ron"));
    }

    #[test]
    fn mod_serde() {
        let cryo_rounds = Mod {
            name: "Cryo Rounds".to_owned(),
            stats: vec![ModEffect::Elemental(Element::Primary(PrimaryElement::Cold), 0.9).into()],
        };

        let text = ron::ser::to_string_pretty(&cryo_rounds, ron::ser::PrettyConfig::default())
            .expect("Mods should be ron-serializable.");

        std::fs::write("test.ron", text.as_str()).expect("Test file should be writable.");
        let text = std::fs::read_to_string("test.ron").expect("Test file should be readable.");

        let parsed =
            ron::de::from_str::<Mod>(text.as_str()).expect("Mods should be ron-deserializable.");

        assert_eq!(cryo_rounds, parsed);
    }
}
