use std::{
    collections::HashMap,
    fs::OpenOptions,
    path::{Path, PathBuf},
};

use crate::{damage::*, enemy::Faction};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum ModEffect {
    Physical(Ips, f32),
    Elemental(Element, f32),
    Bane(Faction, f32),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Mod {
    pub name: String,
    pub effects: Vec<ModEffect>,
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
        let lib = ModLibrary::load(Path::new("data/mods.ron"));

        // let expected = Mod {
        //     name: "Cryo Rounds".to_owned(),
        //     effects: vec![ModEffect::Elemental(
        //         Element::Primary(PrimaryElement::Cold),
        //         0.9,
        //     )],
        // };
        
        

    }

    #[test]
    fn mod_serde() {
        let cryo_rounds = Mod {
            name: "Cryo Rounds".to_owned(),
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
