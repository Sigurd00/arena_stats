use log::Level::{Debug, Trace, Warn};
use log::{debug, log_enabled, trace, warn};

use crate::game::{Game, GameType};
use crate::team::Comp;
use std::collections::HashMap;

pub fn start(games: Vec<Game>) -> bool {
    generate_teamcomp_buckets(&games);
    calculate_rating_change(&games);
    true
}
#[allow(clippy::type_complexity)]
pub fn generate_teamcomp_buckets(
    games: &[Game],
) -> (HashMap<&Comp, Vec<&Game>>, HashMap<&Comp, Vec<&Game>>) {
    let mut friendly_team_comps: HashMap<&Comp, Vec<&Game>> = HashMap::new();
    let mut enemy_team_comps: HashMap<&Comp, Vec<&Game>> = HashMap::new();
    for game in games.iter() {
        friendly_team_comps
            .entry(&game.friendly_team.comp)
            .or_insert_with(|| vec![game])
            .push(game);
        enemy_team_comps
            .entry(&game.enemy_team.comp)
            .or_insert_with(|| vec![game])
            .push(game);
    }
    if log_enabled!(Debug) {
        debug!("Printing all the unique comps!");
        for (comp, _game) in friendly_team_comps.iter() {
            debug!("{:?}", comp);
        }
        debug!("Done printing unique comps!");
    }
    (friendly_team_comps, enemy_team_comps)
}

pub fn calculate_rating_change(games: &[Game]) -> (i32, i32) {
    let mut twos_rating = 0;
    let mut threes_rating = 0;

    for game in games.iter() {
        if game.is_rated {
            if log_enabled!(Trace) {
                trace!("rating change: {:?}", game.rating_change);
            }
            match game.game_type {
                GameType::Twos => twos_rating += game.rating_change,
                GameType::Threes => threes_rating += game.rating_change,
                GameType::Other => {
                    if log_enabled!(Warn) {
                        log_player_missing(game);
                    }
                    if game.friendly_team.players.len() == 3 || game.enemy_team.players.len() == 3 {
                        threes_rating += game.rating_change;
                    } else {
                        twos_rating += game.rating_change;
                    }
                }
            }
        }
    }
    debug!("2v2: {:?}, 3v3: {:?}", twos_rating, threes_rating);
    (twos_rating, threes_rating)
}

fn log_player_missing(game: &Game) {
    if game.friendly_team.players.len() < game.enemy_team.players.len() {
        warn!(
            "Someone on your team left the game, player(s) found in the game: {:?} ",
            game.friendly_team.players
        )
    } else {
        warn!(
            "Someone on the enemy team left, player(s) found in the game: {:?} ",
            game.enemy_team.players
        )
    }
}
