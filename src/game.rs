use chrono::{DateTime, Utc, Duration, NaiveDateTime};
use csv::StringRecord;

use crate::{team::Team, player::Player};

pub enum GameMap {
    ALLMAPS, //TODO: Get a list of all arena maps
}
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
        let mut friendly_players:Vec<Player>;
        let mut enemy_players:Vec<Player>;

        
        Game {
            timestamp: parse_timestamp(&record[0]),
            map: GameMap::ALLMAPS,
            //friendly_team: Team::new()),
            friendly_team: todo!(),
            enemy_team: todo!(),
            duration: todo!(),
            victory: todo!(),
            killing_blows: todo!(),
            damage: todo!(),
            healing: todo!(),
            honor: todo!(),
            rating_change: todo!(),
            is_rated: todo!(),
        }
    }
}

fn parse_timestamp(timestamp: &str) -> DateTime<Utc> {
    let timestamp = timestamp.parse::<i64>().unwrap();
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    //TODO: maybe format timestamp here.
    datetime
}
