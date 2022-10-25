use std::collections::HashMap;

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
        self.comp
    }

    pub fn games(&self) -> &[&Game] {
        self.games.as_slice()
    }

    pub fn add(&mut self, game: &'a Game) {
        self.games.push(game);
        self.len = self.games.len();
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
