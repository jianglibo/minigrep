use chrono::NaiveDateTime;
use super::schema::fs_change_log;

#[derive(Queryable)]
pub struct FsChangeLog {
    pub id: i32,
    pub file_name: String,
    pub new_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub modified_at: Option<NaiveDateTime>,
    pub notified_at: Option<NaiveDateTime>,
    pub size: i32,
}

#[derive(Insertable)]
#[table_name="fs_change_log"]
pub struct NewFsChangeLog<'a> {
    pub file_name: &'a str,
    pub new_name: Option<&'a str>,
    pub created_at: NaiveDateTime,
    pub modified_at: Option<NaiveDateTime>,
    pub notified_at: Option<NaiveDateTime>,
    pub size: i32,
}