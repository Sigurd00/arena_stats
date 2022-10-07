use crate::player::Player;

#[derive(Debug)]
pub struct Team {
    pub players: Vec<Player>,
    pub mmr: i32,
}
