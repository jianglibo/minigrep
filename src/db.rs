use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv::dotenv;
use std::env;
use crate::models::*;

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
    use crate::schema::fs_change_log;

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



#[cfg(test)]
mod tests {
    use crate::schema::fs_change_log::dsl::*;
    use super::*;

    #[test]
    fn test_list_fcl() {
        let connection = establish_connection();
        println!("{}", "xxxxxxxxxxxxxxxxxxxxxxxxx");
        let results = fs_change_log
            // .filter(published.eq(true))
            .limit(5)
            .load::<FsChangeLog>(&connection)
            .expect("Error loading posts");

        println!("{}", "xxxxxxxxxxxxxxxxxxxxxxxxx1");
        println!("Displaying {} posts", results.len());
        for fcl in results {
            println!("{}", fcl.file_name);
            println!("----------\n");
            println!("{}", fcl.size);
        }
        assert!(false);
    }
}