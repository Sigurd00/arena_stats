use crate::player::Player;

pub struct Team {
    players: Vec<Player>,
    mmr: i32,
}

impl Team {
    pub fn new(players: Vec<Player>, mmr: i32) -> Self {
        Team {
            players,
            mmr,
        }
    }
}