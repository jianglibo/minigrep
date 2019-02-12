use crate::models::fs_change_log_model::{FsChangeLog, NewFsChangeLog};
// use crate::schema::fs_change_log;
use ::actix::prelude::*;
use ::actix_web::*;
use ::diesel::prelude::*;
use ::diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};
use ::diesel::SqliteConnection;
use crate::error::WatchError;
// use std::sync::Arc;
// use std::vec::Vec;

// https://github.com/diesel-rs/diesel/blob/master/diesel/src/r2d2.rs

type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
type SqlitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn init_pool(database_url: &str) -> Result<SqlitePool, PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub struct DbExecutor(pub SqlitePool);

impl DbExecutor {
    pub fn get_conn(&self) -> Result<SqlitePooledConnection, Error> {
        self.0.get().map_err(|e| error::ErrorInternalServerError(e))
    }
}

pub struct StopMe();

impl Message for StopMe {
    type Result = ();
}

// impl Message for NewFsChangeLog {
//     type Result = Result<FsChangeLog, Error>;
// }

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<StopMe> for DbExecutor {
    type Result = ();
    fn handle(&mut self, _: StopMe, _:&mut Self::Context) -> Self::Result {
        System::current().stop();
        ()
    }
}

impl Handler<NewFsChangeLog> for DbExecutor {
    // type Result = Result<FsChangeLog, Error>;
    type Result = Result<(), WatchError>;

    fn handle(&mut self, msg: NewFsChangeLog, _: &mut Self::Context) -> Self::Result {
        use crate::schema::fs_change_log::dsl::*;

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::insert_into(fs_change_log)
            .values(&msg)
            .execute(conn)
            .map_err(|_| WatchError::Unknown)?;

        // let items = fs_change_log
        //     .filter(id.eq(&id))
        //     .load::<FsChangeLog>(conn)
        //     .map_err(|_| WatchError::Unknown)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    pub enum ParseError {
        VariantNotFound,
    }

    impl std::fmt::Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            // We could use our macro here, but this way we don't take a dependency on the
            // macros crate.
            match self {
                &ParseError::VariantNotFound => write!(f, "Matching variant not found"),
            }
        }
    }

    impl std::error::Error for ParseError {
        fn description(&self) -> &str {
            match self {
            &ParseError::VariantNotFound => {
                "Unable to find a variant of the given enum matching the string given. Matching \
                 can be extended with the Serialize attribute and is case sensitive."
            }
        }
        }
    }

    #[test]
    fn test_enum() {
        use std::fmt;
        use std::str::FromStr;
        enum AnEnum {
            A,
            B(String),
        }

        impl fmt::Display for AnEnum {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    AnEnum::A => write!(f, "A"),
                    AnEnum::B(ref sv) => write!(f, "{}:{}", "B", sv),
                }
            }
        }

        impl std::str::FromStr for AnEnum {
            type Err = ParseError;

            fn from_str(s: &str) -> Result<AnEnum, Self::Err> {
                match s {
                    "A" => Ok(AnEnum::A),
                    bs if s.starts_with("B:") => {
                        let ss: Vec<&str> = bs.split(":").collect();
                        Ok(AnEnum::B(String::from(ss[1])))
                    }
                    _ => Err(ParseError::VariantNotFound),
                }
            }
        }

        let name: String = AnEnum::A.to_string();
        assert_eq!(name, "A");

        let name: String = AnEnum::B(String::from("hello")).to_string();
        assert_eq!(name, "B:hello");

        let an: Result<AnEnum, ParseError> = AnEnum::from_str("B:abc");

        assert!(an.is_ok());
    }
}

// pub fn get_db_pool() -> SqlitePool {
//     dotenv::dotenv().ok();
//     let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let manager = ConnectionManager::<SqliteConnection>::new(database_url);
//     Pool::builder()
//         .max_size(10)
//         .build(manager)
//         .expect("Failed to create pool.")
// }