#[macro_use]
extern crate lazy_static;

use std::env;
use std::process;
use minigrep;

mod directory_access;
mod test_diesel;
mod mysql;
mod fxiture_util;
mod test_string;
mod test_fun;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}