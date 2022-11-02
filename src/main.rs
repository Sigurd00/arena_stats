use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;

use arena_stats::analyser;
use arena_stats::parser;

fn main() {
    env::set_var("RUST_LOG", "none");
    env_logger::init();

    let file_path = match get_first_arg() {
        Ok(file_path) => file_path,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    let games = match parser::parse_games(file_path) {
        Ok(games) => games,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    analyser::start(&games);
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
