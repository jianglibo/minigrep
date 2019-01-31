use crate::db::DB_POOL;
use crate::schema::fs_change_log;
use crate::schema::fs_change_log::dsl as fs_c_log_dsl;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use notify::DebouncedEvent;
use std::convert::From;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

#[derive(Queryable)]
pub struct FsChangeLog {
    pub id: i32,
    pub event_type: String,
    pub file_name: String,
    pub new_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub modified_at: Option<NaiveDateTime>,
    pub notified_at: NaiveDateTime,
    pub size: i32,
}

#[derive(Insertable)]
#[table_name = "fs_change_log"]
pub struct NewFsChangeLog<'a> {
    pub event_type: &'a str,
    pub file_name: &'a str,
    pub new_name: Option<&'a str>,
    pub created_at: NaiveDateTime,
    pub modified_at: Option<NaiveDateTime>,
    pub notified_at: NaiveDateTime,
    pub size: i32,
}

impl FsChangeLog {
    pub fn create<'a>(
        event_type: &'a str,
        file_name: &'a str,
        new_name: Option<&'a str>,
        created_at: NaiveDateTime,
        modified_at: Option<NaiveDateTime>,
        size: i32,
    ) -> usize {
        let new_fs_change_log = NewFsChangeLog {
            event_type,
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

impl<'a> From<&'a DebouncedEvent> for NewFsChangeLog<'a> {
    fn from(de: &'a DebouncedEvent) -> Self {
        let bd = |src_pbuf: Option<&'a PathBuf>, dst_pbuf: Option<&'a PathBuf>, en: &'a str| -> NewFsChangeLog<'a> {
            let ppbuf = dst_pbuf.or(src_pbuf);

            let fs_meta = match ppbuf {
                Some(pbuf) => std::fs::metadata(pbuf).ok(),
                None => None,
            };

            NewFsChangeLog {
                file_name: {
                    match src_pbuf {
                        Some(pbuf) => pbuf.to_str().unwrap_or(""),
                        None => "",
                    }
                },
                event_type: en,
                new_name: {
                    match dst_pbuf {
                        Some(pbuf) => Some(pbuf.to_str().unwrap_or("")),
                        None => None,
                    }
                },
                created_at: if let Some(meta) = fs_meta {
                    let now = meta.created().unwrap_or(SystemTime::now());
                    Utc::now().naive_utc()
                } else {
                    Utc::now().naive_utc()
                 },
                modified_at: None,
                notified_at: Utc::now().naive_utc(),
                size: 55,
            }
        };
        match de {
            DebouncedEvent::NoticeWrite(pbuf) => bd(Some(&pbuf), None, "NoticeWrite"),
            DebouncedEvent::NoticeRemove(pbuf) => bd(Some(&pbuf), None, "NoticeRemove"),
            DebouncedEvent::Create(pbuf) => bd(Some(&pbuf), None, "Create"),
            DebouncedEvent::Write(pbuf) => bd(Some(&pbuf), None, "Write"),
            DebouncedEvent::Chmod(pbuf) => bd(Some(&pbuf), None, "Chmod"),
            DebouncedEvent::Remove(pbuf) => bd(Some(&pbuf), None, "Remove"),
            DebouncedEvent::Rename(src, dst) => bd(Some(&src), Some(&dst), "Rename"),
            DebouncedEvent::Rescan => bd(None, None, "Rescan"),
            _ => bd(None, None, "Other"),
        }
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

        let num: usize = FsChangeLog::create(
            "NoticeRemove",
            r"c:\abc.txt",
            None,
            Utc::now().naive_utc(),
            None,
            0,
        );
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
}
