use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Weapon {
    ty: WeaponType,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum WeaponType {
    Primary(PrimaryWeaponType),
    Secondary(SecondaryWeaponType),
    Melee(MeleeWeaponType),
    Exalted(Box<WeaponType>),
    Archwing(ArchwingWeaponType),
    Companion(CompanionWeaponType),
    Modular(ModularWeaponType),
    Railjack(RailjackWeaponType),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum PrimaryWeaponType {
    ArmCannon,
    Bow,
    Crossbow,
    Launcher,
    Rifle,
    Shotgun,
    SniperRifle,
    Speargun,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum SecondaryWeaponType {
    Crossbow,
    DualPistols,
    DualShotguns,
    Pistol,
    ShotgunSidearm,
    Thrown,
    Tome,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum MeleeWeaponType {
    AssaultSaw,
    BladeAndWhip,
    Claws,
    Dagger,
    DualDaggers,
    DualNikanas,
    DualSwords,
    Fist,
    Glaive,
    Gunblade,
    Hammer,
    HeavyBlade,
    HeavyScythe,
    Machete,
    Nikana,
    Nunchaku,
    Polearm,
    Rapier,
    Scythe,
    Sparring,
    Staff,
    SwordAndShield,
    Sword,
    Tonfa,
    TwoHandedNikana,
    Warfan,
    Whip,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum ArchwingWeaponType {
    Archgun,
    DualPistols,
    Launcher,
    Melee,
    Rifle,
    Shotgun,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum CompanionWeaponType {
    Glaive,
    Melee,
    Pistol,
    Rifle,
    Shotgun,
    SniperRifle,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum ModularWeaponType {
    Amp,
    Launcher,
    Melee(MeleeWeaponType),
    Pistol,
    Rifle,
    Shotgun,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum RailjackWeaponType {
    Ordnance,
    Turret,
}
