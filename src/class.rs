use std::str::FromStr;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
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

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Comp {
    size: i32,
    team_classes: Vec<Class>,
}

impl Default for Comp {
    fn default() -> Self {
        Self::new()
    }
}
impl Comp {
    pub fn new() -> Comp {
        Comp {
            size: 0,
            team_classes: vec![],
        }
    }

    pub fn add_class(&mut self, class: Class) {
        self.size += 1;
        self.team_classes.push(class);
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn team_classes(&self) -> &[Class] {
        self.team_classes.as_ref()
    }
}
