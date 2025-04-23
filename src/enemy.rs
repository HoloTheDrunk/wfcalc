use crate::damage::DamageType;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
pub enum Faction {
    Grineer,
    Corpus,
    Corrupted,
    Infested,
    Murmur,
}

pub struct Enemy {
    pub faction: Faction,
    pub weaknesses: HashMap<DamageType, f32>,
}

impl Enemy {
    pub fn weakness_to(&self, damage_type: &DamageType) -> f32 {
        self.weaknesses.get(damage_type).copied().unwrap_or(1.)
    }
}
