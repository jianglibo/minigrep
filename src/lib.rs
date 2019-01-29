use std::error::Error;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_fs_change_log<'a>(
    conn: &SqliteConnection,
    file_name: &'a str,
    new_name: Option<&'a str>,
    created_at: NaiveDateTime,
    modified_at: Option<NaiveDateTime>,
    notified_at: Option<NaiveDateTime>,
    size: i32,
) -> usize {
    //   ) -> FsChangeLog {
    use self::schema::fs_change_log;

    let new_post = NewFsChangeLog {
        file_name,
        new_name,
        created_at,
        modified_at,
        notified_at,
        size,
    };

    diesel::insert_into(fs_change_log::table)
        .values(&new_post)
        // .get_result(conn)
        .execute(conn)
        .expect("Error saving new post")
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[allow(dead_code)]
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

#[allow(dead_code)]
fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use self::models::*;
    use self::schema::fs_change_log::dsl::*;
    use super::*;

    #[test]
    fn test_list_fcl() {
        let connection = establish_connection();
        let results = fs_change_log
            // .filter(published.eq(true))
            .limit(5)
            .load::<FsChangeLog>(&connection)
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
        for fcl in results {
            println!("{}", fcl.file_name);
            println!("----------\n");
            println!("{}", fcl.size);
        }
    }
    #[test]
    fn test_insert_fcl() {}
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn t_array() {
        let mut array: [i32; 3] = [0; 3];
        array[1] = 1;
        array[2] = 2;
        assert_eq!([1, 2], &array[1..]);
        for x in &array {
            print!("{} ", x);
        }
    }

    #[test]
    fn t_visit_dir() {
        let path = Path::new("./src");
        let dp = |f: &DirEntry| print!("{:?}", f.file_name());
        assert!(visit_dirs(path, &dp).is_ok());
    }
}
