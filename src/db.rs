use crate::models::fs_change_log_model::{FsChangeLog, NewFsChangeLog};
// use crate::schema::fs_change_log;
use ::actix::prelude::*;
use actix_web::*;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, PoolError};
use diesel::SqliteConnection;
// use std::sync::Arc;
// use std::vec::Vec;

// https://github.com/diesel-rs/diesel/blob/master/diesel/src/r2d2.rs

// lazy_static! {
//     pub static ref DB_POOL: Arc<Pool<ConnectionManager<SqliteConnection>>> = {
//         dotenv::dotenv().ok();
//         let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         let manager = ConnectionManager::<SqliteConnection>::new(database_url);
//         Arc::new(Pool::builder().max_size(10).build(manager).unwrap())
//     };
// }
type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
type SqlitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn init_pool(database_url: &str) -> Result<SqlitePool, PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager)
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

pub struct DbExecutor(pub SqlitePool);

impl DbExecutor {
    pub fn get_conn(&self) -> Result<SqlitePooledConnection, Error> {
        self.0.get().map_err(|e| error::ErrorInternalServerError(e))
    }
}

/// This is only message that this actor can handle, but it is easy to extend
/// number of messages.
pub struct CreateFsChangeLog {
    pub event_type: String,
    pub file_name: String,
    pub new_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub modified_at: Option<NaiveDateTime>,
    pub notified_at: NaiveDateTime,
    pub size: i64,
}

impl Message for CreateFsChangeLog {
    type Result = Result<FsChangeLog, Error>;
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<CreateFsChangeLog> for DbExecutor {
    type Result = Result<FsChangeLog, Error>;

    fn handle(&mut self, msg: CreateFsChangeLog, _: &mut Self::Context) -> Self::Result {
        use crate::schema::fs_change_log::dsl::*;
        let new_name_v: Option<&str> = msg.new_name.map(|v| v.as_str());
        let new_fs_change_log = NewFsChangeLog {
            event_type: &msg.event_type,
            file_name: &msg.file_name,
            new_name: new_name_v,
            created_at: msg.created_at,
            modified_at: msg.modified_at,
            notified_at: Utc::now().naive_utc(),
            size: msg.size,
        };

        let conn: &SqliteConnection = &self.0.get().unwrap();

        diesel::insert_into(fs_change_log)
            .values(&new_fs_change_log)
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting person"))?;

        let mut items = fs_change_log
            .filter(id.eq(&id))
            .load::<FsChangeLog>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;

        Ok(items.pop().unwrap())
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
