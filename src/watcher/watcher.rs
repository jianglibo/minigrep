use crate::app_state::AppState;
use crate::models::fs_change_log_model::NewFsChangeLog;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread;
use futures::Future;
use actix::prelude::*;


// struct DirWatcher {
//     app_state: AppState,
// }

pub fn watch(watch_target: &str, app_state: AppState) -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(watch_target, RecursiveMode::Recursive)?;

    let ep = move |event: DebouncedEvent| {
                            // let res: Result<NewFsChangeLog, actix::MailboxError> = app_state.db.send(NewFsChangeLog::from(&event)).from_err()
                            // pub fn send<M>(&self, msg: M) -> Request<A, M>
                            // res: actix::prelude::Request<db::DbExecutor, models::fs_change_log_model::NewFsChangeLog>
                            let result_future = app_state.db.send(NewFsChangeLog::from(&event));
Arbiter::spawn(
                                    result_future.map(|res| {
            match res {
                Ok(result) => (),
                Err(err) => println!("Got error: {}", err),
            }
        })
        .map_err(|e| {
            println!("Actor is probably died: {}", e);
        }));


                            // .from_err()
                            // .and_then(|res| match res {
                            //     Ok(fs_change_log_item) => Ok(()),
                            //     Err(_) => Ok(()),
                            // });
                    // match event {
                    //     DebouncedEvent::NoticeWrite(path_buf) => {
                    //         DebouncedEvent::NoticeWrite;
                    //         print!("notice_write: {:?}", path_buf);
                    //     },
                    //     DebouncedEvent::NoticeRemove(path_buf) => print!("{:?}", path_buf),
                    //     DebouncedEvent::Create(path_buf) => print!("{:?}", path_buf),
                    //     DebouncedEvent::Write(path_buf) => print!("{:?}", path_buf),
                    //     DebouncedEvent::Chmod(path_buf) => print!("{:?}", path_buf),
                    //     DebouncedEvent::Remove(path_buf) => print!("{:?}", path_buf),
                    //     DebouncedEvent::Rename(src, dst) => print!("{:?}=>{:?}", src, dst),
                    //     DebouncedEvent::Rescan => println!("rescan"),
                    //     _ =>  println!("{:?}", event)
                    // }

    };

    let handle = thread::spawn(move || {
                    match rx.recv() {
                Ok(event) => ep(event),
                Err(e) => println!("watch error: {:?}", e),
            }
    });

    Ok(())


    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    // if how_much_times == 0 {
    //     loop {
    //         match rx.recv() {
    //             Ok(event) => ep(event),
    //             Err(e) => println!("watch error: {:?}", e),
    //         }
    //     }
    // } else {
    //     for _ in 0..how_much_times {
    //         match rx.recv() {
    //             Ok(event) => ep(event),
    //             Err(e) => println!("watch error: {:?}", e),
    //         }
    //     }
    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir};
    use std::fs::File;
    use std::io::{Write};
    // #[test]

    // fn test_w() {
    //     let dir = tempdir().unwrap();
    //     let file_path = dir.path().join("my-temporary-note.txt");
    //     let mut file = File::create(file_path).unwrap();
    //     writeln!(file, "Brian was here. Briefly.").unwrap();
    //     if let Err(e) = watch(dir.path().to_str().unwrap(), 2) {
    //         println!("error: {:?}", e)
    //     }
    // }
}