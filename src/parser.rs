use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, Utc};
use log::debug;

use crate::{
    class::Class,
    player::{Player, Realm},
    team::Team,
};

pub fn parse_timestamp(timestamp: &str) -> DateTime<Utc> {
    let timestamp = timestamp.parse::<i64>().unwrap();
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
    let friendly_team = Team {
        players: prase_players(friendly_team),
        mmr: friendly_team_mmr.parse::<i32>().unwrap(),
    };
    let enemy_team = Team {
        players: prase_players(enemy_team),
        mmr: enemy_team_mmr.parse::<i32>().unwrap(),
    };
    (friendly_team, enemy_team)
}

pub fn prase_players(team_string: String) -> Vec<Player> {
    //DEMONHUNTER-Havoc-Demongubbe-TheMaelstrom,PRIEST-Holy-Skli
    let mut players: Vec<Player> = vec![];
    let player_strings: Vec<&str> = team_string.split(',').collect();
    for player in player_strings {
        players.push(parse_player(player));
    }
    debug!("Found players: {:?} ", players);
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
