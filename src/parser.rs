use std::{error::Error, ffi::OsString, fs::File, str::FromStr};

use chrono::{DateTime, NaiveDateTime, Utc};
use log::{debug, log_enabled, trace, warn, Level};

use crate::{
    class::Class,
    game::{Game, GameType},
    player::{Player, Realm},
    team::Team,
};

pub fn parse_games(file_path: OsString) -> Result<Vec<Game>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);
    let mut games: Vec<Game> = vec![];
    for result in rdr.records() {
        let record = result?;
        if log_enabled!(Level::Trace) {
            trace!("Record: {:?}", record);
        }
        let game = Game::new(record);
        if log_enabled!(Level::Trace) {
            trace!("Game: {:?}", game);
        }
        //TODO: Maybe do something with the games that are being thrown away?
        if let GameType::Other = game.game_type {
            log_player_missing(&game);
        } else {
            games.push(game);
        }
    }
    debug!("Total games: {}\n", games.len());
    Ok(games)
}

pub fn parse_timestamp(timestamp: &str) -> DateTime<Utc> {
    let timestamp = timestamp.parse::<i64>().expect("Failed to parse timestamp");
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    //TODO: maybe format timestamp here.
    datetime
}

pub fn parse_teams(
    friendly_team: String,
    enemy_team: String,
    friendly_team_mmr: String,
    enemy_team_mmr: String,
) -> (Team, Team) {
    let friendly_team = Team::new(
        parse_players(friendly_team),
    friendly_team_mmr.parse::<i32>().expect("Failed to parse friendly team mmr"),
    );
    let enemy_team = Team::new(
        parse_players(enemy_team),
        enemy_team_mmr.parse::<i32>().expect("Failed to parse enemy team mmr"),
    );
    (friendly_team, enemy_team)
}

pub fn parse_players(team_string: String) -> Vec<Player> {
    //DEMONHUNTER-Havoc-Demongubbe-TheMaelstrom,PRIEST-Holy-Skli
    let mut players: Vec<Player> = vec![];
    let player_strings: Vec<&str> = team_string.split(',').collect();
    for player in player_strings {
        players.push(parse_player(player));
    }
    trace!("Found players: {:?} ", players);
    players
}

fn parse_player(player_string: &str) -> Player {
    let mut something = player_string.split('-');
    Player {
        class: Class::from_str(something.next().unwrap()).unwrap(),
        spec: something.next().unwrap().to_string(),
        name: something.next().unwrap().to_string(),
        realm: parse_potential_realm(something.next()),
    }
}

fn parse_potential_realm(maybe_realm: Option<&str>) -> Option<Realm> {
    maybe_realm.map(|_realm| Realm::Draenor)
}

fn log_player_missing(game: &Game) {
    if game.friendly_team.players.len() < game.enemy_team.players.len() {
        warn!(
            "Someone on your team left the game, player(s) found in the game: {:?}, game has been deleted",
            game.friendly_team.players
        )
    } else {
        warn!(
            "Someone on the enemy team left, player(s) found in the game: {:?}, game has been deleted",
            game.enemy_team.players
        )
    }
}
