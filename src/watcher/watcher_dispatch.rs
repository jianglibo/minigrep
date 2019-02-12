use crate::app_state::AppState;
use crate::models::fs_change_log_model::NewFsChangeLog;
use crate::error::WatchError;
use actix::prelude::*;

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
    type Result = Result<(), WatchError>;

    fn handle(&mut self, msg: NewFsChangeLog, _: &mut Self::Context) -> Self::Result {
        match self.app_state.db.try_send(msg) {
            Ok(_) => Ok(()),
            Err(_) => Err(WatchError::Unknown),
        }
    }
}

impl StreamHandler<NewFsChangeLog, ()> for WatcherDispatch {
    fn handle(&mut self, item: NewFsChangeLog, _: &mut Context<WatcherDispatch>) {
        match self.app_state.db.try_send(item) {
            Ok(_) => (),
            Err(_) => (),
        }
    }
}