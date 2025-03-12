use std::{fmt, i32};

use chrono::{DateTime, Duration, Utc};
use csv::StringRecord;

use crate::team::Comp;
use crate::{
    parser::{self, parse_teams},
    team::Team,
};

#[derive(Debug, Clone)]
pub enum GameMap {
    ALLMAPS, //TODO: Get a list of all arena maps
}

#[derive(Clone)]
pub enum GameType {
    Twos,
    Threes,
    Other,
}

impl GameType {
    fn from_player_count(player_count: i32) -> Self {
        match player_count {
            4 => GameType::Twos,
            6 => GameType::Threes,
            _ => GameType::Other,
        }
    }
}

impl fmt::Debug for GameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            GameType::Twos => write!(f, "GameType: [2v2]"),
            GameType::Threes => write!(f, "GameType: [3v3]"),
            GameType::Other => write!(f, "Yikes, someone left this game"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub timestamp: DateTime<Utc>,
    pub map: GameMap,
    pub game_type: GameType,
    pub friendly_team: Team,
    pub enemy_team: Team,
    pub duration: Duration,
    pub victory: bool,
    pub killing_blows: i32,
    pub damage: i32,
    pub healing: i32,
    pub honor: i32,
    pub rating_change: i32,
    pub is_rated: bool,
}

impl Game {
    pub fn new(record: StringRecord) -> Self {
        let (friendly_team, enemy_team) = parse_teams(
            record[3].to_string(),
            record[4].to_string(),
            record[12].to_string(),
            record[13].to_string(),
        );
        let player_count = record[2].parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Error: Invalid player count in record: {:?}", record);
            0  // Default to 0
        });
        Game {
            timestamp: parser::parse_timestamp(&record[0]),
            map: GameMap::ALLMAPS,
            friendly_team,
            enemy_team,
            game_type: GameType::from_player_count(player_count),
            duration: Duration::seconds(record[5].parse::<i64>().expect("Failed to parse game duration")),
            victory: matches!(&record[6], "true"),
            killing_blows: record[7].parse::<i32>().expect("Failed to parse killing blows"),
            damage: record[8].parse::<i32>().expect("Failed to parse damage"),
            healing: record[9].parse::<i32>().expect("Failed to parse healing"),
            honor: record[10].parse::<i32>().expect("Failed to parse honor"),
            rating_change: record[11].parse::<i32>().expect("Failed to parse rating change"),
            is_rated: matches!(&record[15], "true"),
        }
    }
}

pub fn print_all_comps(comps: Vec<Comp>) {
    for comp in comps {
        for class in comp.team_classes() {
            print!("{:? }", class);
        }
        println!();
    }
}
