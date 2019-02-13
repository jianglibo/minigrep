
use crate::db::{DbExecutor, self};
use crate::watcher::watcher_dispatch::WatcherDispatch;
use crate::watcher::watcher::DirWatcher;
use ::actix::{Addr, SyncArbiter, Arbiter, StreamHandler};
use crate::app_state::AppState;

const LINE_START: &str = "for-easy-installer-client-use-start";
const LINE_END: &str = "for-easy-installer-client-use-end";

pub fn send_string_to_client(str_content: &str) {
    println!("{}", LINE_START);
    println!("{}", str_content);
    println!("{}", LINE_END);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SoftwareDescription {
    pub package_url: String,
    pub local_name: String,
}

pub fn create_actors_env() -> (Addr<DbExecutor>, Addr<WatcherDispatch>) {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let wd = std::env::var("WATCH_DIR").expect("WATCH_DIR must be set.");
    create_actors(database_url, 3, wd)
}


pub fn create_actors(database_url: String, db_threads: usize, watch_dir: String) -> (Addr<DbExecutor>, Addr<WatcherDispatch>) {
    if !database_url.contains(":") {
        let db_path = std::path::Path::new(&database_url);
        match db_path.parent() {
            Some(p) => {
                if !p.exists() {
                    panic!("database folder {:?} does't exists.", p)
                }
            },
            None => panic!("database folder does't exists.")
        }
    }
    let pool = db::init_pool(&database_url).unwrap();

    let db_addr = SyncArbiter::start(db_threads, move || DbExecutor(pool.clone()));
    let db_addr1 = db_addr.clone();

    let wd_addr = Arbiter::start(move |ctx| {
        // use futures::stream::once;
        let dw = DirWatcher::new(&watch_dir);
        WatcherDispatch::add_stream(dw, ctx);
        let app_state = AppState { db: db_addr1 };
        WatcherDispatch {
            app_state,
        }
    });
    (db_addr, wd_addr)
}