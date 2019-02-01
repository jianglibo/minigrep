use std::env;
use std::path::{PathBuf};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::SqliteConnection;
use crate::db;

#[allow(dead_code)]
pub fn get_fixture_file(postfix: &[&str], canonicalize: bool) -> std::io::Result<PathBuf> {
    let mut path_result = env::current_dir()?;
    path_result = path_result.join("fixtures");
    for x in postfix {
        // x is &&str
        path_result = path_result.join(x);
    }
    if canonicalize {
        Ok(path_result.canonicalize()?)
    } else {
        Ok(path_result)
    }
}

#[allow(dead_code)]
pub fn print_stars<T: AsRef<str>>(v: T) {
    println!("xxxxxxxxxxxxxx{}xxxxxxxxxxxx", v.as_ref());
}


pub fn get_connect() -> PooledConnection<ConnectionManager<SqliteConnection>> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<SqliteConnection>> = db::init_pool(&database_url).unwrap();
    pool.get().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_fixture_file() {
        let f = super::get_fixture_file(&["mysql", "my.cnf"], true).unwrap();
        assert!(f.exists());
        let metadata = std::fs::metadata(f).unwrap();
        assert_eq!(metadata.len(), 987);
    }
}

