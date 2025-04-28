use wfcalc::mods::*;

use std::time::Duration;

fn main() {
    let galvanized_chamber = Mod {
        name: "Galvanized Chamber".to_owned(),
        stats: vec![
            ModEffect::Multishot(0.8).into(),
            ModStat {
                trigger: Some(Trigger::Kill),
                stacking: Some(Stacking {
                    max: 5,
                    behaviour: StackingBehaviour::Timed {
                        duration: Duration::from_secs(20),
                        timeout: TimeoutBehaviour::Reduce(ReduceAmount::Flat(1)),
                        resets_on_stack: true,
                    },
                }),
                effect: ModEffect::Multishot(0.3),
            },
        ],
    };

    let text = ron::ser::to_string_pretty(&galvanized_chamber, ron::ser::PrettyConfig::default())
        .expect("Mods should be ron-serializable.");

    println!("{text}");
}
