use crate::models::fs_change_log_model::NewFsChangeLog;
use futures::{Async, Poll, Stream};
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

pub struct DirWatcher {
    // app_state: AppState,
    rx: std::sync::mpsc::Receiver<DebouncedEvent>,
}

impl DirWatcher {
    pub fn new(watch_target: &str /*, app_state: AppState*/) -> DirWatcher {
        let watch_path = std::path::Path::new(watch_target);
        if !(watch_path.is_dir() && watch_path.exists()) {
            panic!("watch target {} does't exists.", watch_target);
        }
        let (tx, rx) = channel();

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher
            .watch(watch_target, RecursiveMode::Recursive)
            .unwrap();
        DirWatcher {
            // app_state,
            rx,
        }
    }
}

impl Stream for DirWatcher {
    type Item = NewFsChangeLog;
    // The stream will never yield an error
    type Error = ();

    fn poll(&mut self) -> Poll<Option<NewFsChangeLog>, ()> {
        match self.rx.try_recv() {
            Ok(de) => Ok(Async::Ready(Some(NewFsChangeLog::from(&de)))),
            Err(tre) => {
                println!("{:?}", tre);
                Ok(Async::NotReady)
            }
        }
    }
}

// pub fn watch<T: AsRef<std::path::Path>>(
//     watch_target: T,
//     app_state: AppState,
// ) -> notify::Result<()> {
//     // Create a channel to receive the events.
//     let (tx, rx) = channel();

//     // Automatically select the best implementation for your platform.
//     // You can also access each implementation directly e.g. INotifyWatcher.
//     let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

//     // Add a path to be watched. All files and directories at that path and
//     // below will be monitored for changes.
//     watcher.watch(watch_target, RecursiveMode::Recursive)?;

//     let ep = move |event: DebouncedEvent| {
//         // let res: Result<NewFsChangeLog, actix::MailboxError> = app_state.db.send(NewFsChangeLog::from(&event)).from_err()
//         // pub fn send<M>(&self, msg: M) -> Request<A, M>
//         // res: actix::prelude::Request<db::DbExecutor, models::fs_change_log_model::NewFsChangeLog>
//         app_state.db.do_send(NewFsChangeLog::from(&event));
//         let result_future = app_state.db.send(NewFsChangeLog::from(&event));
//         Arbiter::spawn(
//             result_future
//                 .map(|res| match res {
//                     Ok(result) => (),
//                     Err(err) => println!("Got error: {}", err),
//                 })
//                 .map_err(|e| {
//                     println!("Actor is probably died: {}", e);
//                 }),
//         );

//         // .from_err()
//         // .and_then(|res| match res {
//         //     Ok(fs_change_log_item) => Ok(()),
//         //     Err(_) => Ok(()),
//         // });
//         // match event {
//         //     DebouncedEvent::NoticeWrite(path_buf) => {
//         //         DebouncedEvent::NoticeWrite;
//         //         print!("notice_write: {:?}", path_buf);
//         //     },
//         //     DebouncedEvent::NoticeRemove(path_buf) => print!("{:?}", path_buf),
//         //     DebouncedEvent::Create(path_buf) => print!("{:?}", path_buf),
//         //     DebouncedEvent::Write(path_buf) => print!("{:?}", path_buf),
//         //     DebouncedEvent::Chmod(path_buf) => print!("{:?}", path_buf),
//         //     DebouncedEvent::Remove(path_buf) => print!("{:?}", path_buf),
//         //     DebouncedEvent::Rename(src, dst) => print!("{:?}=>{:?}", src, dst),
//         //     DebouncedEvent::Rescan => println!("rescan"),
//         //     _ =>  println!("{:?}", event)
//         // }
//     };

//     // let handle =
//     thread::spawn(move || match rx.recv() {
//         Ok(event) => ep(event),
//         Err(e) => println!("watch error: {:?}", e),
//     });

//     Ok(())

//     // This is a simple loop, but you may want to use more complex logic here,
//     // for example to handle I/O.
//     // if how_much_times == 0 {
//     //     loop {
//     //         match rx.recv() {
//     //             Ok(event) => ep(event),
//     //             Err(e) => println!("watch error: {:?}", e),
//     //         }
//     //     }
//     // } else {
//     //     for _ in 0..how_much_times {
//     //         match rx.recv() {
//     //             Ok(event) => ep(event),
//     //             Err(e) => println!("watch error: {:?}", e),
//     //         }
//     //     }
//     //     Ok(())
//     // }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common_util;
    use crate::db;
    use crate::fixture_util::get_connect;
    use crate::models::fs_change_log_model::FsChangeLog;
    use ::actix::{Arbiter, System};
    use ::futures::Future;
    use chrono::Utc;
    use std::fs::File;
    use std::io::Write;
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;
    use tempfile::tempdir;
    use crate::common_message;

    // #[test]
    // fn test_w() {
    //     let dir = tempdir().unwrap();
    //     let file_path = dir.path().join("my-temporary-note.txt");
    //     let (sys, addr, pool) = run_system();
    //     watch(&dir, AppState { db: addr.clone()}).unwrap();

    //     // thread::spawn(move || {
    //     //     let _ = sys.run();
    //     // });

    //     let mut file = File::create(file_path).unwrap();
    //     writeln!(file, "Brian was here. Briefly.").unwrap();
    //     sleep(Duration::new(2, 0));

    //     assert_eq!(FsChangeLog::all(10, &pool.get().unwrap()).unwrap().len(), 2);
    //     // addr.send()

    // }
    #[test]
    fn test_arbit() {
        // assert!(true);
        dotenv::dotenv().ok();
        // let database_url = String::from(":memory"); //std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let test_watch_dir = std::env::var("TEST_WATCH_DIR").expect("TEST_WATCH_DIR must be set");
        System::run(move || {
            let (db_addr, _) = common_util::create_actors(database_url, 1, test_watch_dir);
            let db_addr1 = db_addr.clone();
            let mut deleted = 0usize;
            let ft = db_addr
                .send(common_message::RemoveAll::FsChangeLog)
                .from_err()

                .and_then(move |res| {
                    deleted = res.unwrap();
                    // return future again.
                    db_addr1.send(db::ListFsChangeLog { total: 10 })
                    .from_err()
                    .and_then(|res| match res {
                        Ok(fs_logs) => {
                            for i in &fs_logs {
                                println!("{}", i.file_name);
                            }
                            assert_eq!(fs_logs.len(), 0);
                            Ok(())
                        }
                        Err(_) => Err(::actix_web::error::ErrorInternalServerError("abc")),
                    })
                });

            Arbiter::spawn(ft.map_err(|e| {
                println!("Actor is probably died: {}", e);
            }));
            let nfs = NewFsChangeLog {
                event_type: String::from("NoticeRemove"),
                file_name: String::from(r"c:\abc.txt"),
                new_name: None,
                created_at: Utc::now().naive_utc(),
                modified_at: None,
                notified_at: Utc::now().naive_utc(),
                size: -1,
            };
            db_addr.do_send(nfs);
            db_addr.do_send(db::StopMe {});
        });
        assert_eq!(FsChangeLog::all(10, &get_connect()).unwrap().len(), 1);
    }
}
