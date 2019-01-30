use crate::models::*;
// use crate::schema::fs_change_log;
use crate::schema::fs_change_log::dsl as fs_c_log_dsl;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use std::sync::Arc;
use std::vec::Vec;

// https://github.com/diesel-rs/diesel/blob/master/diesel/src/r2d2.rs

lazy_static! {
    static ref DB_POOL: Arc<Pool<ConnectionManager<SqliteConnection>>> = {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        Arc::new(Pool::builder().max_size(10).build(manager).unwrap())
    };
}

impl FsChangeLog {
    pub fn create<'a>(
        file_name: &'a str,
        new_name: Option<&'a str>,
        created_at: NaiveDateTime,
        modified_at: Option<NaiveDateTime>,
        size: i32,
    ) -> usize {
        let new_fs_change_log = NewFsChangeLog {
            file_name,
            new_name,
            created_at,
            modified_at,
            notified_at: Utc::now().naive_utc(),
            size,
        };

        diesel::insert_into(fs_c_log_dsl::fs_change_log)
            .values(&new_fs_change_log)
            .execute(&DB_POOL.get().unwrap())
            .expect("Error saving new post")
    }

    pub fn delete_all() -> usize {
        diesel::delete(fs_c_log_dsl::fs_change_log)
            .execute(&DB_POOL.get().unwrap())
            .expect("Error deleting posts")
    }

    pub fn find_all(num: i64) -> Vec<FsChangeLog> {
        fs_c_log_dsl::fs_change_log
            .limit(num)
            .load::<FsChangeLog>(&DB_POOL.get().unwrap())
            .expect("Error loading posts")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_list_fcl() {
        FsChangeLog::delete_all();
        assert_eq!(FsChangeLog::find_all(5).len(), 0);

        let num: usize = FsChangeLog::create(r"c:\abc.txt", None, Utc::now().naive_utc(), None, 0);
        assert_eq!(num, 1);
        let items: Vec<FsChangeLog> = FsChangeLog::find_all(5);
        assert_eq!(items.len(), 1);
        assert_eq!(&items[0].file_name, r"c:\abc.txt");
    }

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
