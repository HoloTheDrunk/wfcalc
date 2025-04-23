use serde::{Deserialize, Serialize};

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