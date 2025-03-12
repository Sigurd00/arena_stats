use std::collections::HashMap;

use log::Level::Debug;
use log::{debug, log_enabled};
use percentage::{Percentage, PercentageDecimal};

use crate::{game::Game, team::Comp};

pub type GameBuckets = HashMap<Comp, GameBucket>;

pub struct GameBucket {
    comp: Comp,
    games: Vec<Game>,
    wins: usize,
    losses: usize,
    len: usize,
    winrate: PercentageDecimal,
    should_update: bool,
}

impl GameBucket {
    pub fn new(comp: Comp) -> Self {
        Self {
            comp,
            games: vec![],
            wins: 0,
            losses: 0,
            len: 0,
            winrate: Percentage::from_decimal(0.0),
            should_update: false,
        }
    }

    pub fn comp(&self) -> &Comp {
        &self.comp
    }

    pub fn games(&self) -> &[Game] {
        self.games.as_slice()
    }

    pub fn add(&mut self, game: Game) {
        let victory = game.victory;
        self.games.push(game);
        self.len = self.games.len();

        if victory {
            self.wins += 1;
        } else {
            self.losses += 1;
        }
        self.should_update = true;
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

    pub fn winrate(&mut self) -> &PercentageDecimal {
        if self.should_update {
            self.update();
        }
        &self.winrate
    }

    fn update(&mut self) {
        self.winrate = Percentage::from_decimal(self.wins as f64 / self.games.len() as f64);
        if log_enabled!(Debug) {
            debug!(
                "Games won: {}, Games lost: {}, winrate: {:.2}",
                self.wins,
                self.len() - self.wins,
                self.winrate.value()
            );
        }
        self.should_update = false;
    }
}
