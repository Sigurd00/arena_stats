use crate::player::Player;

pub struct Game {
    pub size: i32,
    pub players: Vec<Player>,
}

impl Game {
    /// Creates a new [`Game`].
    fn new(size: i32, players: Vec<Player>) -> Game {
        return Game {
            size: size,
            players: players,
        };
    }
}
