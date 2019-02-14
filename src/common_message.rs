use ::actix::Message;
use ::diesel::prelude::QueryResult;

#[derive(Debug)]
pub enum RemoveAll {
    FsChangeLog,
}


impl Message for RemoveAll {
    type Result = QueryResult<usize>;
}