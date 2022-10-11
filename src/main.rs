use std::env;
use std::process;

use arena_stats::run::run;

fn main() {
    env::set_var("RUST_LOG", "off");
    env_logger::init();
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
