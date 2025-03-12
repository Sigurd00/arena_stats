use std::fmt;

use crate::class::Class;
use crate::player::Player;

#[derive(Debug, Clone)]
pub struct Team {
    pub players: Vec<Player>,
    pub comp: Comp,
    pub mmr: i32,
}

impl Team {
    pub fn new(players: Vec<Player>, mmr: i32) -> Self {
        Team {
            comp: Comp::new(&players),
            players,
            mmr,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq, Debug, PartialOrd, Ord)]
pub struct Comp {
    size: i32,
    team_classes: Vec<Class>,
}

impl fmt::Display for Comp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.team_classes)
    }
}

impl Comp {
    pub fn new(players: &Vec<Player>) -> Comp {
        let mut team_classes: Vec<Class> = vec![];
        for player in players {
            team_classes.push(player.class);
        }
        Comp {
            size: team_classes.len() as i32,
            team_classes,
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
