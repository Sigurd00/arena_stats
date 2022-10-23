use std::{collections::btree_map::Iter, mem::take, marker::PhantomData};

use log::debug;
use percentage::{Percentage, PercentageInteger};

use crate::{team::Comp, game::Game};

pub struct GameBucket<'a> {
    comp: Comp,
    games: Vec<&'a Game>,
    pub len: usize,
    pub winrate: PercentageInteger,
}

impl GameBucket<'_> {
    fn new(comp: Comp) -> Self {
        Self {
            len: games.len(),
            comp,
            winrate: calculate_winrate(&games),
            games,
        }
    }

    pub fn comp(&self) -> &Comp {
        &self.comp
    }

    pub fn games(&self) -> &Vec<Game> {
        self.games.as_ref()
    }

    pub fn add(&mut self, game:Game) {
        self.games.push(game);
    }
}

impl<'a> Iterator for GameBucket<'a> {
    type Item = &'a Game;

    fn next(&mut self) -> Option<&'a Game> {
        self.games.iter().next()
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
