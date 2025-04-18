use chrono::Duration;
use log::Level::{Debug, Trace};
use log::{debug, log_enabled, trace, warn};
use pad::PadStr;

use crate::game::{Game, GameType};
use crate::game_bucket::{GameBucket, GameBuckets};
use crate::team::Comp;
use std::cmp::Reverse;
use std::collections::HashMap;

const COMP_THRESHOLD: usize = 20;

//TODO: Find a better way to do this. ie "game bucket is slice for all games"
/// Used for functions that either use all the games or use game buckets.
enum Games<'a> {
    AllGames(&'a [Game]),
    GameBucket(&'a GameBucket),
}

pub fn start(games: &[Game]) -> bool {
    let (mut friendly_comps, mut enemy_comps) = put_games_into_buckets(games);
    //maybe do all of these calculations as "jobs" such that we dont iterate over games 5-10 times, but do all the things that need to be done in one iteration instead
    calculate_rating_change(games);
    calculate_average_gametime(&Games::AllGames(games));
    debug!("{:?}, {:?}", friendly_comps.len(), enemy_comps.len());
    for (comp, games) in enemy_comps.iter_mut() {
        let duration = calculate_average_gametime(&Games::GameBucket(&games));
        debug!(
            "Comp: {} - Average Duration: {} Minutes, {} Seconds - PlayCount: {}, Winrate: {:.2}",
            comp.to_string().pad_to_width(25),
            duration.num_minutes(),
            duration.num_seconds() / 60,
            games.len(),
            games.winrate().value(),
        )
    }

    let most_common = most_common_team(friendly_comps, 5);
    for (comp, playcount) in most_common.iter() {
        println!("{} - {}", comp, playcount);
    }
    /* for (_comp, games) in enemy_comps.iter() {
        calculate_average_gametime(Games::GameBucket(games));
    } */
    true
}

#[allow(clippy::type_complexity)]
pub fn put_games_into_buckets(games: &[Game]) -> (GameBuckets, GameBuckets) {
    let mut friendly_team_comps: GameBuckets = HashMap::new();
    let mut enemy_team_comps: GameBuckets = HashMap::new();
    for game in games.iter() {
        friendly_team_comps
            .entry(game.friendly_team.comp.clone())
            .or_insert_with(|| GameBucket::new(game.friendly_team.comp.clone()))
            .add(game.clone());
        enemy_team_comps
            .entry(game.enemy_team.comp.clone())
            .or_insert_with(|| GameBucket::new(game.enemy_team.comp.clone()))
            .add(game.clone());
    }
    debug!(
        "total comps before prune: {:?}",
        friendly_team_comps.len() + enemy_team_comps.len()
    );
    friendly_team_comps.retain(|_, v| v.len() >= COMP_THRESHOLD);
    enemy_team_comps.retain(|_, v| v.len() >= COMP_THRESHOLD);
    if log_enabled!(Debug) {
        debug!("Printing all the unique comps!");
        for (comp, _game) in friendly_team_comps.iter() {
            debug!("{:?}", comp);
        }
        for (comp, _game) in enemy_team_comps.iter() {
            debug!("{:?}", comp);
        }
        debug!(
            "total comps after prune: {:?}",
            friendly_team_comps.iter().len() + enemy_team_comps.iter().len()
        );
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
                    warn!("A game that is missing is included in the calculations, maybe have a look into this!");
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

fn calculate_average_gametime(game_bucket: &Games) -> Duration {
    let mut total_duration = Duration::seconds(0);
    let total_games: i32;
    match game_bucket {
        Games::AllGames(games) => {
            for game in games.iter() {
                total_duration = total_duration + game.duration;
            }
            total_games = games.len() as i32;
        }
        Games::GameBucket(game_bucket) => {
            for game in game_bucket.games().iter() {
                total_duration = total_duration + game.duration;
            }
            total_games = game_bucket.len() as i32;
        }
    }
    let average_duration = total_duration / total_games;
    debug!(
        "{} minutes, {} seconds",
        average_duration.num_minutes(),
        average_duration.num_seconds() % 60
    );
    average_duration
}

fn most_common_team(game_buckets: HashMap<Comp, GameBucket>, count: usize) -> Vec<(Comp, usize)> {
    let mut sorted: Vec<_> = game_buckets
        .into_iter()
        .map(|(comp, bucket)| (comp, bucket.len()))
        .collect();

    sorted.sort_by_key(|&(_, len)| Reverse(len));
    sorted.truncate(count);
    sorted
}
