use chrono::{DateTime, Duration, Utc};
use csv::StringRecord;
use itertools::Itertools;

use crate::{
    class::Comp,
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
            friendly_team,
            enemy_team,
            duration: Duration::seconds(record[5].parse::<i64>().unwrap()),
            victory: matches!(&record[6], "true"),
            killing_blows: record[7].parse::<i32>().unwrap(),
            damage: record[8].parse::<i32>().unwrap(),
            healing: record[9].parse::<i32>().unwrap(),
            honor: record[10].parse::<i32>().unwrap(),
            rating_change: record[11].parse::<i32>().unwrap(),
            is_rated: matches!(&record[15], "true"),
        }
    }
}

pub fn generate_teamcomp_buckets(games: Vec<Game>) -> (Vec<Comp>, Vec<Comp>) {
    let mut friendly_comps: Vec<Comp> = vec![]; //TODO: Figure out how we connect the friendly_comps games with the correct enemy_comps games
    let mut enemy_comps: Vec<Comp> = vec![];
    for game in games {
        let mut comp = Comp::new();
        for player in game.friendly_team.players {
            comp.add_class(player.class);
        }
        friendly_comps.push(comp);
        let mut comp = Comp::new();
        for player in game.enemy_team.players {
            comp.add_class(player.class);
        }
        enemy_comps.push(comp);
    }
    friendly_comps = friendly_comps.into_iter().unique().collect();
    (friendly_comps, enemy_comps)
}

pub fn print_all_comps(comps: Vec<Comp>) {
    for comp in comps {
        for class in comp.team_classes() {
            print!("{:? }", class);
        }
        println!();
    }
}
