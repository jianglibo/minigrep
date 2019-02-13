use crate::app_state::AppState;
use crate::models::fs_change_log_model::NewFsChangeLog;
use actix::prelude::*;
use ::diesel::prelude::QueryResult;

/**
 * This is an Actor which consume dirwatcher stream.
 */
pub struct WatcherDispatch {
    pub app_state: AppState,
}

impl Actor for WatcherDispatch {
    type Context = Context<Self>;
}

impl  Handler<NewFsChangeLog> for WatcherDispatch {
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: NewFsChangeLog, _: &mut Self::Context) -> Self::Result {
        let file_name = msg.file_name.clone();
        match self.app_state.db.try_send(msg) {
            Ok(_) => (),
            Err(_) => error!("process {}'s event failed.", file_name)
        }
        Ok(1)
    }
}

impl StreamHandler<NewFsChangeLog, ()> for WatcherDispatch {
    fn handle(&mut self, item: NewFsChangeLog, _: &mut Context<WatcherDispatch>) {
        let file_name = item.file_name.clone();
        match self.app_state.db.try_send(item) {
            Ok(_) => (),
            Err(_) => error!("process {}'s event failed.", file_name)
        }
    }
}