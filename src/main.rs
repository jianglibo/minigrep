#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate notify;
extern crate yaml_rust;

use std::env;
use std::process;
use minigrep;

mod directory_access;
mod test_diesel;
mod mysql;
mod fxiture_util;
mod test_string;
mod test_fun;
mod borg;
mod common_util;
mod watcher;
mod db;
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;



fn main() {
    // let args: Vec<String> = env::args().collect();

    // let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    // if let Err(e) = minigrep::run(config) {
    //     println!("Application error: {}", e);
    //     process::exit(1);
    // }
}