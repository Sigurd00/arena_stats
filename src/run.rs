use std::{env, error::Error, ffi::OsString, fs::File};

use log::{log_enabled, trace, Level};

use crate::{analyser, game::Game};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
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
        games.push(game);
    }
    analyser::start(games);
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
