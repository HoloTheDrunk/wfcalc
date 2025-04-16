#![feature(let_chains)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum ModEffect {
    Physical(Ips, f32),
    Elemental(Element, f32),
    Bane(Faction, f32),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
pub enum Faction {
    Grineer,
    Corpus,
    Corrupted,
    Infested,
    Murmur,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Mod {
    name: Option<String>,
    effects: Vec<ModEffect>,
}

pub struct Hit {
    base_damage: HashMap<DamageType, f32>,
    mods: Vec<Mod>,
    enemy: Enemy,
}

pub struct Enemy {
    faction: Faction,
    weaknesses: HashMap<DamageType, f32>,
}

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone, Copy)]
#[serde(untagged)]
pub enum DamageType {
    Physical(Ips),
    Elemental(Element),
    Special(Special),
}

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Ips {
    Impact,
    Puncture,
    Slash,
}

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone, Copy)]
#[serde(untagged)]
pub enum Element {
    Primary(PrimaryElement),
    Secondary(SecondaryElement),
}

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Special {
    Void,
    Tau,
    True,
}

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone, Copy)]
pub enum PrimaryElement {
    Cold,
    Heat,
    Toxin,
    Electricity,
}

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone, Copy)]
pub enum SecondaryElement {
    Blast,
    Viral,
    Magnetic,
    Gas,
    Radiation,
    Corrosive,
}

impl Hit {
    pub fn total_base(&self) -> f32 {
        self.base_damage.values().sum()
    }

    pub fn scale(&self) -> f32 {
        self.total_base() / 16.
    }

    pub fn quantize(&self, value: f32) -> f32 {
        let scale = self.scale();
        (value / scale).round() * scale
    }

    pub fn weakness_to(&self, damage_type: &DamageType) -> f32 {
        self.enemy
            .weaknesses
            .get(damage_type)
            .copied()
            .unwrap_or(1.)
    }

    pub fn contributions(&self) -> HashMap<DamageType, f32> {
        let mut physical_calculator = PhysicalCalculator::new(None);
        let mut elemental_calculator = ElementalCalculator::new(None);
        let mut bane = 1.;

        for r#mod in self.mods.iter() {
            for effect in r#mod.effects.iter() {
                match effect {
                    ModEffect::Physical(ips, value) => physical_calculator.add(*ips, *value),
                    ModEffect::Elemental(elem, value) => elemental_calculator.add(*elem, *value),
                    ModEffect::Bane(faction, value) if self.enemy.faction == *faction => {
                        bane += value
                    }
                    ModEffect::Bane(_, _) => println!("Inactive bane"),
                }
            }
        }

        let base = [Ips::Impact, Ips::Puncture, Ips::Slash]
            .into_iter()
            .flat_map(|ips| {
                self.base_damage
                    .get(&DamageType::Physical(ips))
                    .map(|v| self.quantize(*v) * self.weakness_to(&DamageType::Physical(ips)))
                    .map(|v| (DamageType::Physical(ips), v))
            });

        let physical = physical_calculator
            .finalize()
            .into_iter()
            .map(|(ips, value)| {
                let value = self
                    .base_damage
                    .get(&DamageType::Physical(ips))
                    .map(|base| {
                        self.quantize(value * base) * self.weakness_to(&DamageType::Physical(ips))
                    })
                    .unwrap_or(0.);
                (DamageType::Physical(ips), value)
            });

        let total_base = self.total_base();
        let elemental = elemental_calculator
            .finalize()
            .into_iter()
            .map(|(elem, value)| {
                (
                    DamageType::Elemental(elem),
                    self.quantize(value * total_base)
                        * self.weakness_to(&DamageType::Elemental(elem)),
                )
            });

        let mut result = HashMap::new();

        base.chain(physical)
            .chain(elemental)
            .for_each(|(dt, value)| *result.entry(dt).or_insert(0.) += value * bane);

        result
    }

    pub fn total_quantized(&self) -> f32 {
        let mut physical_calculator = PhysicalCalculator::new(None);
        let mut elemental_calculator = ElementalCalculator::new(None);
        let mut bane = 1.;

        for r#mod in self.mods.iter() {
            for effect in r#mod.effects.iter() {
                match effect {
                    ModEffect::Physical(ips, value) => physical_calculator.add(*ips, *value),
                    ModEffect::Elemental(elem, value) => elemental_calculator.add(*elem, *value),
                    ModEffect::Bane(faction, value) if self.enemy.faction == *faction => {
                        bane += value
                    }
                    ModEffect::Bane(_, _) => println!("Inactive bane"),
                }
            }
        }

        self.contributions().values().sum()
    }
}

struct PhysicalCalculator {
    ips: Vec<(Ips, f32)>,
    // TODO: Add lich damage calculation (based on total or base?)
    _lich: Option<(Ips, f32)>,
}

impl PhysicalCalculator {
    pub fn new(lich: Option<(Ips, f32)>) -> Self {
        Self {
            ips: vec![],
            _lich: lich,
        }
    }

    pub fn add(&mut self, ips: Ips, value: f32) {
        self.ips.push((ips, value));
    }

    pub fn finalize(self) -> HashMap<Ips, f32> {
        let mut result: HashMap<Ips, f32> = Default::default();

        for (ips, value) in self.ips {
            *result.entry(ips).or_insert(0.) += value;
        }

        result
    }
}

struct ElementalCalculator {
    primary: Vec<(PrimaryElement, f32)>,
    secondary: Vec<(SecondaryElement, f32)>,
    _lich: Option<(Element, f32)>,
}

macro_rules! map {
    {$($key:expr => $value:expr),* $(,)?} => {
        HashMap::from([$(($key, $value)),*])
    };
}

impl ElementalCalculator {
    // TODO: LazyStatic
    fn combinations() -> HashMap<SecondaryElement, (PrimaryElement, PrimaryElement)> {
        use PrimaryElement::*;
        use SecondaryElement::*;

        map! {
            Blast => (Cold, Heat),
            Viral => (Cold, Toxin),
            Magnetic => (Cold, Electricity),
            Gas => (Heat, Toxin),
            Radiation => (Heat, Electricity),
            Corrosive => (Toxin, Electricity),
        }
    }

    fn new(lich: Option<(Element, f32)>) -> Self {
        Self {
            primary: vec![],
            secondary: vec![],
            _lich: lich,
        }
    }

    fn add(&mut self, ty: Element, value: f32) {
        match ty {
            Element::Primary(e) => self.primary.push((e, value)),
            Element::Secondary(e) => self.secondary.push((e, value)),
        }
    }

    fn finalize(self) -> HashMap<Element, f32> {
        let mut result: HashMap<Element, f32> = Default::default();

        for (elem, value) in self.primary {
            Self::add_primary(&mut result, elem, value);
        }

        for (elem, value) in self.secondary {
            Self::add_secondary(&mut result, elem, value);
        }

        result
    }

    fn add_primary(result: &mut HashMap<Element, f32>, ty: PrimaryElement, value: f32) {
        let map = Self::combinations();

        let mut was_combined = false;
        for (secondary, (l, r)) in map.iter() {
            if *l != ty && *r != ty {
                continue;
            }
            let other_elem = Element::Primary(*if *l == ty { r } else { l });

            // Secondary element is already registered.
            if let Some(cur) = result.get_mut(&Element::Secondary(*secondary)) {
                *cur += value;
                was_combined = true;
            }
            // Combine with the other required primary element to produce the
            // secondary element.
            else if let Some(other_value) = result.get(&other_elem)
                && *other_value > 0.
            {
                result.insert(Element::Secondary(*secondary), value + *other_value);
                result.insert(other_elem, 0.);
                was_combined = true;
            }
        }

        if was_combined {
            return;
        }

        *result.entry(Element::Primary(ty)).or_insert(0.) += value;
    }

    fn add_secondary(result: &mut HashMap<Element, f32>, ty: SecondaryElement, value: f32) {
        *result.entry(Element::Secondary(ty)).or_insert(0.) += value;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_float_eq::assert_f32_near;

    #[test]
    fn basic_ips() {
        let hit = Hit {
            base_damage: [
                (DamageType::Physical(Ips::Impact), 30.),
                (DamageType::Physical(Ips::Puncture), 30.),
                (DamageType::Physical(Ips::Slash), 40.),
            ]
            .into(),
            mods: vec![],
            enemy: Enemy {
                faction: Faction::Infested,
                weaknesses: [(DamageType::Physical(Ips::Slash), 1.5)].into(),
            },
        };

        assert_f32_near!(hit.total_base(), 100.);
        assert_f32_near!(hit.scale(), 6.25);
        assert_f32_near!(hit.total_quantized(), 118.75);
    }

    #[test]
    fn nagantaka_prime_wiki() {
        let cryo_rounds = Mod {
            name: Some("Cryo Rounds".to_owned()),
            effects: vec![ModEffect::Elemental(
                Element::Primary(PrimaryElement::Cold),
                0.9,
            )],
        };

        let malignant_force = Mod {
            name: Some("Malignant Force".to_owned()),
            effects: vec![ModEffect::Elemental(
                Element::Primary(PrimaryElement::Toxin),
                0.6,
            )],
        };

        let hellfire = Mod {
            name: Some("Hellfire".to_owned()),
            effects: vec![ModEffect::Elemental(
                Element::Primary(PrimaryElement::Heat),
                0.9,
            )],
        };

        let piercing_caliber = Mod {
            name: Some("Piercing Caliber".to_owned()),
            effects: vec![ModEffect::Physical(Ips::Puncture, 1.2)],
        };

        let valence_formation_gas = Mod {
            name: Some("Valence Formation - Gas".to_owned()),
            effects: vec![ModEffect::Elemental(
                Element::Secondary(SecondaryElement::Gas),
                2.,
            )],
        };

        let hit = Hit {
            base_damage: [
                (DamageType::Physical(Ips::Impact), 1.7),
                (DamageType::Physical(Ips::Puncture), 15.6),
                (DamageType::Physical(Ips::Slash), 155.7),
            ]
            .into(),
            mods: vec![
                cryo_rounds,
                malignant_force,
                hellfire,
                piercing_caliber,
                valence_formation_gas,
            ],
            enemy: Enemy {
                faction: Faction::Infested,
                weaknesses: HashMap::new(),
            },
        };

        assert_f32_near!(hit.total_base(), 173.);
        assert_f32_near!(hit.scale(), 10.8125);

        let contributions = hit.contributions();
        assert_f32_near!(
            *contributions
                .get(&DamageType::Physical(Ips::Impact))
                .unwrap(),
            0.
        );
        assert_f32_near!(
            *contributions
                .get(&DamageType::Physical(Ips::Puncture))
                .unwrap(),
            10.8125 + 21.625
        );
        assert_f32_near!(
            *contributions
                .get(&DamageType::Physical(Ips::Slash))
                .unwrap(),
            151.375
        );
        assert_f32_near!(
            *contributions
                .get(&DamageType::Elemental(Element::Secondary(
                    SecondaryElement::Viral
                )))
                .unwrap(),
            259.5
        );
        assert_f32_near!(
            *contributions
                .get(&DamageType::Elemental(Element::Primary(
                    PrimaryElement::Heat
                )))
                .unwrap(),
            151.375
        );
        assert_f32_near!(
            *contributions
                .get(&DamageType::Elemental(Element::Secondary(
                    SecondaryElement::Gas
                )))
                .unwrap(),
            346.
        );

        assert_f32_near!(hit.total_quantized(), 940.6875);
    }

    #[test]
    fn mod_serde() {
        let cryo_rounds = Mod {
            name: Some("Cryo Rounds".to_owned()),
            effects: vec![ModEffect::Elemental(
                Element::Primary(PrimaryElement::Cold),
                0.9,
            )],
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
