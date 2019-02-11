/// State with DbExecutor address
use crate::db::DbExecutor;
use actix::prelude::Addr;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}