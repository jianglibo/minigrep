use crate::app_state::AppState;
use crate::models::fs_change_log_model::NewFsChangeLog;
use actix::prelude::*;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use futures::{Stream, Poll, Async, Future};

/**
 * This is an Actor which consume dirwatcher stream.
 */
pub struct WatcherDispatch {
    app_state: AppState,
}

impl Actor for WatcherDispatch {
    type Context = Context<Self>;
}

impl  Handler<NewFsChangeLog> for WatcherDispatch {

}

impl StreamHandler<NewFsChangeLog, ()> for WatcherDispatch {

}