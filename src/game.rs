use chrono::{DateTime, Duration, Utc};
use csv::StringRecord;

use crate::{
    parser::{self, parse_teams},
    team::Team,
};

#[derive(Debug)]
pub enum GameMap {
    ALLMAPS, //TODO: Get a list of all arena maps
}

#[derive(Debug)]
pub struct Game {
    pub timestamp: DateTime<Utc>,
    pub map: GameMap,
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
        Game {
            timestamp: parser::parse_timestamp(&record[0]),
            map: GameMap::ALLMAPS,
            //friendly_team: Team::new()),
            friendly_team: friendly_team,
            enemy_team: enemy_team,
            duration: Duration::seconds(record[5].parse::<i64>().unwrap()),
            victory: match &record[6] {
                "true" => true,
                _ => false,
            },
            killing_blows: record[7].parse::<i32>().unwrap(),
            damage: record[8].parse::<i32>().unwrap(),
            healing: record[9].parse::<i32>().unwrap(),
            honor: record[10].parse::<i32>().unwrap(),
            rating_change: record[11].parse::<i32>().unwrap(),
            is_rated: match &record[15] {
                "true" => true,
                _ => false,
            },
        }
    }
}
