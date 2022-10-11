use chrono::Duration;
use log::Level::{Debug, Trace, Warn};
use log::{debug, log_enabled, trace, warn};
use percentage::{Percentage, PercentageInteger};

use crate::game::{Game, GameType};
use crate::team::Comp;
use std::collections::HashMap;

pub fn start(games: Vec<Game>) -> bool {
    generate_teamcomp_buckets(&games);
    //maybe do all of these calculations as "jobs" such that we dont iterate over games 5-10 times, but do all the things that need to be done in one iteration instead
    calculate_rating_change(&games);
    calculate_winrate(&games);
    calculate_average_gametime(&games);
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

fn calculate_winrate(games: &[Game]) -> PercentageInteger {
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

fn calculate_average_gametime(games: &[Game]) -> Duration {
    let mut total_duration = Duration::seconds(0);
    for game in games.iter() {
        total_duration = total_duration + game.duration;
    }
    let average_duration = total_duration / games.len() as i32;
    debug!(
        "Average game time: {} minutes, {} seconds",
        average_duration.num_minutes(),
        average_duration.num_seconds() % 60
    );
    average_duration
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
