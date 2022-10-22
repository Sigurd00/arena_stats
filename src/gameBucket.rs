use log::debug;
use percentage::{Percentage, PercentageInteger};

use crate::{game::Game, team::Comp};

pub struct GameBucket<'a> {
    pub comp: Comp,
    pub games: &'a Vec<Game>,
    pub len: usize,
    pub winrate: PercentageInteger,
}

impl<'a> GameBucket<'a> {
    fn new(comp: Comp, games: &'a Vec<Game>) -> Self {
        Self {
            len: games.len(),
            comp,
            games,
            winrate: calculate_winrate(&games),
        }
    }
}

pub fn calculate_winrate(games: &[Game]) -> PercentageInteger {
    let mut wins = 0;
    for game in games.iter() {
        if game.victory {
            wins += 1;
        }
    }
    let winrate = Percentage::from_decimal(wins as f64 / games.len() as f64);
    debug!(
        "Games won: {}, Games lost: {}, winrate: {:.2}",
        wins,
        games.len() - wins,
        winrate.value()
    );
    Percentage::from(wins / games.len())
}
