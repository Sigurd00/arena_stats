use std::str::FromStr;

#[derive(Debug)]
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
