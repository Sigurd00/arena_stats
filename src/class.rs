use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Eq, Hash, PartialEq, Copy, PartialOrd, Ord)]
pub enum Class {
    Warrior,
    Hunter,
    Paladin,
    Priest,
    Rogue,
    Shaman,
    Warlock,
    Mage,
    Monk,
    Druid,
    DemonHunter,
    DeathKnight,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Class::Warrior => write!(f, "Warrior"),
            Class::Hunter => write!(f, "Hunter"),
            Class::Paladin => write!(f, "Paladin"),
            Class::Priest => write!(f, "Priest"),
            Class::Rogue => write!(f, "Rogue"),
            Class::Shaman => write!(f, "Shaman"),
            Class::Warlock => write!(f, "Warlock"),
            Class::Mage => write!(f, "Mage"),
            Class::Monk => write!(f, "Monk"),
            Class::Druid => write!(f, "Druid"),
            Class::DemonHunter => write!(f, "Demon Hunter"),
            Class::DeathKnight => write!(f, "Death Knight"),
        }
    }
}

impl FromStr for Class {
    type Err = ();

    fn from_str(input: &str) -> Result<Class, Self::Err> {
        match input {
            "WARRIOR" => Ok(Class::Warrior),
            "HUNTER" => Ok(Class::Hunter),
            "PALADIN" => Ok(Class::Paladin),
            "PRIEST" => Ok(Class::Priest),
            "ROGUE" => Ok(Class::Rogue),
            "SHAMAN" => Ok(Class::Shaman),
            "WARLOCK" => Ok(Class::Warlock),
            "MAGE" => Ok(Class::Mage),
            "MONK" => Ok(Class::Monk),
            "DRUID" => Ok(Class::Druid),
            "DEMONHUNTER" => Ok(Class::DemonHunter),
            "DEATHKNIGHT" => Ok(Class::DeathKnight),
            _ => Err(()),
        }
    }
}
