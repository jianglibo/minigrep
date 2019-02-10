use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;


struct DirWatcher {
    app_state: AppState,
}

fn watch(watch_target: &str, how_much_times: u8) -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(watch_target, RecursiveMode::Recursive)?;

    let ep = |event: DebouncedEvent| {
                    match event {
                        DebouncedEvent::NoticeWrite(path_buf) => {
                            DebouncedEvent::NoticeWrite;
                            print!("notice_write: {:?}", path_buf);
                        },
                        DebouncedEvent::NoticeRemove(path_buf) => print!("{:?}", path_buf),
                        DebouncedEvent::Create(path_buf) => print!("{:?}", path_buf),
                        DebouncedEvent::Write(path_buf) => print!("{:?}", path_buf),
                        DebouncedEvent::Chmod(path_buf) => print!("{:?}", path_buf),
                        DebouncedEvent::Remove(path_buf) => print!("{:?}", path_buf),
                        DebouncedEvent::Rename(src, dst) => print!("{:?}=>{:?}", src, dst),
                        DebouncedEvent::Rescan => println!("rescan"),
                        _ =>  println!("{:?}", event)
                    }

    };

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    if how_much_times == 0 {
        loop {
            match rx.recv() {
                Ok(event) => ep(event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    } else {
        for _ in 0..how_much_times {
            match rx.recv() {
                Ok(event) => ep(event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir};
    use std::fs::File;
    use std::io::{Write};
    #[test]

    fn test_w() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = File::create(file_path).unwrap();
        writeln!(file, "Brian was here. Briefly.").unwrap();
        if let Err(e) = watch(dir.path().to_str().unwrap(), 2) {
            println!("error: {:?}", e)
        }
    }
}