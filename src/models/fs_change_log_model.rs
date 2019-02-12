use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use notify::DebouncedEvent;
use std::convert::From;
use std::path::PathBuf;
use std::time::{Duration, UNIX_EPOCH};
use diesel::SqliteConnection;
use crate::schema::{
    fs_change_log, fs_change_log::dsl::{fs_change_log as all_fs_change_log}
};
use ::actix::prelude::Message;
use crate::error::WatchError;


#[derive(Queryable, Serialize)]
pub struct FsChangeLog {
    pub id: i32,
    pub event_type: String,
    pub file_name: String,
    pub new_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub modified_at: Option<NaiveDateTime>,
    pub notified_at: NaiveDateTime,
    pub size: i64,
}

#[derive(Insertable, Deserialize, Debug, Serialize)]
#[table_name = "fs_change_log"]
pub struct NewFsChangeLog {
    pub event_type: String,
    pub file_name: String,
    pub new_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub modified_at: Option<NaiveDateTime>,
    pub notified_at: NaiveDateTime,
    pub size: i64,
}

impl Message for NewFsChangeLog {
    type Result = Result<(), WatchError>;
}

impl FsChangeLog {
    pub fn create<'a>(new_fs_cl: NewFsChangeLog, conn: &SqliteConnection) -> usize {
        diesel::insert_into(fs_change_log::table)
            .values(&new_fs_cl)
            .execute(conn)
            .expect("Error saving new post")
    }

    pub fn delete_all(conn: &SqliteConnection) -> QueryResult<usize> {
        diesel::delete(all_fs_change_log)
            .execute(conn)
    }

    pub fn all(num: i64, conn: &SqliteConnection) -> QueryResult<Vec<FsChangeLog>> {
        all_fs_change_log
            .limit(num)
            .load::<FsChangeLog>(conn)
    }

    pub fn find_by_id(id: i32, conn: &SqliteConnection) -> QueryResult<Option<Vec<FsChangeLog>>> {
        all_fs_change_log
            .filter(fs_change_log::id.eq(&id))
            .load::<FsChangeLog>(conn).map(|items| if items.len() > 0 {
                Some(items)
            } else {
                None
            })
    }
}

impl From<&DebouncedEvent> for NewFsChangeLog {
    fn from(de: &DebouncedEvent) -> Self {
        let bd = |src_path_buf: Option<&PathBuf>,
                  dst_path_buf: Option<&PathBuf>,
                  event_type_str: &str|
         -> NewFsChangeLog {
            let path_buf_opt = dst_path_buf.or(src_path_buf);

            let fs_meta = match path_buf_opt {
                Some(path_buf) => std::fs::metadata(path_buf).ok(),
                None => None,
            };

            let cr_mr_size: (NaiveDateTime, NaiveDateTime, i64) = if let Some(meta) = fs_meta {
                    let _cr = meta.created().unwrap_or(UNIX_EPOCH)
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or(Duration::from_millis(0));
                    let cr = NaiveDateTime::from_timestamp(_cr.as_secs() as i64, _cr.subsec_nanos());
                    let _mr = meta.modified().unwrap_or(UNIX_EPOCH)
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or(Duration::from_millis(0));
                    let mr = NaiveDateTime::from_timestamp(_mr.as_secs() as i64, _mr.subsec_nanos());
                    (cr, mr, meta.len() as i64)
            } else {
                let epoch = NaiveDateTime::from_timestamp(0, 0);
                (epoch, epoch, 0)
            };

            NewFsChangeLog {
                file_name: {
                    match src_path_buf {
                        Some(path_buf) => path_buf.to_str().unwrap_or("").to_owned(),
                        None => String::from("")
                    }
                },
                event_type: event_type_str.to_owned(),
                new_name: {
                    match dst_path_buf {
                        Some(path_buf) => Some(path_buf.to_str().unwrap_or("").to_owned()),
                        None => None,
                    }
                },
                created_at: cr_mr_size.0,
                modified_at: Some(cr_mr_size.1),
                notified_at: Utc::now().naive_utc(),
                size: cr_mr_size.2,
            }
        };
        match de {
            DebouncedEvent::NoticeWrite(path_buf) => bd(Some(&path_buf), None, "NoticeWrite"),
            DebouncedEvent::NoticeRemove(path_buf) => bd(Some(&path_buf), None, "NoticeRemove"),
            DebouncedEvent::Create(path_buf) => bd(Some(&path_buf), None, "Create"),
            DebouncedEvent::Write(path_buf) => bd(Some(&path_buf), None, "Write"),
            DebouncedEvent::Chmod(path_buf) => bd(Some(&path_buf), None, "Chmod"),
            DebouncedEvent::Remove(path_buf) => bd(Some(&path_buf), None, "Remove"),
            DebouncedEvent::Rename(src, dst) => bd(Some(&src), Some(&dst), "Rename"),
            DebouncedEvent::Rescan => bd(None, None, "Rescan"),
            _ => bd(None, None, "Other"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixture_util::{get_connect};
    use chrono::Utc;
    #[test]
    fn test_list_fcl() {
        let conn = &get_connect();
        FsChangeLog::delete_all(conn).unwrap();
        assert_eq!(FsChangeLog::all(5, conn).unwrap().len(), 0);

        let nfs = NewFsChangeLog{
            event_type: String::from("NoticeRemove"),
            file_name: String::from(r"c:\abc.txt"),
            new_name: None,
            created_at: Utc::now().naive_utc(),
            modified_at: None,
            notified_at: Utc::now().naive_utc(),
            size: 0,
        };

        let num: usize = FsChangeLog::create(nfs, conn);
        assert_eq!(num, 1);
        let items: Vec<FsChangeLog> = FsChangeLog::all(5, conn).unwrap();
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
