use log::debug;
use percentage::{Percentage, PercentageInteger};

use crate::{game::Game, team::Comp};

pub struct GameBucket {
    comp: Comp,
    games: Vec<Game>,
    wins: usize,
    losses: usize,
    len: usize,
    winrate: PercentageInteger,
}

impl GameBucket {
    fn new(comp: Comp) -> Self {
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

    pub fn games(&self) -> &Vec<Game> {
        self.games.as_ref()
    }

    pub fn add(&mut self, game: Game) {
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

impl IntoIterator for GameBucket {
    type Item = Game;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.games.into_iter()
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
