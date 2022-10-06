use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

use arena_stats::game::Game;
use log::{info, debug, log_enabled, Level};

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);
    let mut games:Vec<Game> = vec![];
    for result in rdr.records() {
        let record = result?;
        if log_enabled!(Level::Debug){
            debug!("{:?}", record);
        }
        games.push(Game::new(record));
    }
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
