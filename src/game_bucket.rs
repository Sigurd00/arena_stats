
use std::collections::HashMap;

use log::debug;
use percentage::{Percentage, PercentageInteger};

use crate::{game::Game, team::Comp};

pub type GameBuckets<'a> = HashMap<&'a Comp, GameBucket<'a>>;

pub struct GameBucket<'a> {
    comp: &'a Comp,
    games: Vec<&'a Game>,
    wins: usize,
    losses: usize,
    len: usize,
    winrate: PercentageInteger,
}

impl<'a> GameBucket<'a> {
    pub fn new(comp: &'a Comp) -> Self {
        Self {
            comp,
            games: vec![],
            wins: 0,
            losses: 0,
            len: 0,
            winrate: Percentage::from(0),
        }
    }

    pub fn comp(&self) -> &Comp {
        &self.comp
    }

    pub fn games(&self) -> &Vec<&Game> {
        self.games.as_ref()
    }

    pub fn add(&mut self, game: &'a Game) {
        self.games.push(game);
    }

    pub fn wins(&self) -> usize {
        self.wins
    }

    pub fn losses(&self) -> usize {
        self.losses
    }

    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn winrate(&self) -> &PercentageInteger {
        &self.winrate
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
