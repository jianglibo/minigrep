#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate actix;
extern crate actix_web;
extern crate notify;
extern crate serde_yaml;
extern crate yaml_rust;

// use std::env;
// use std::process;
// use minigrep;

mod borg;
mod common_util;
mod db;
mod directory_access;
mod fixture_util;
pub mod models;
mod mysql;
pub mod schema;
mod test_diesel;
mod test_fun;
mod test_string;
mod watcher;
mod app_state;
mod error;

#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate uuid;

// extern crate json;

use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, Error, HttpMessage, HttpRequest, HttpResponse, Json,
    State,
};
use bytes::BytesMut;

use futures::{future, Future, Stream};

use db::DbExecutor;
use app_state::AppState;
use models::fs_change_log_model::NewFsChangeLog;
use watcher::watcher::DirWatcher;
use watcher::watcher_dispatch::WatcherDispatch;



/// Async request handler
// fn add(
//     (name, state): (Path<String>, State<AppState>),
// ) -> FutureResponse<HttpResponse> {
//     // send async `CreateUser` message to a `DbExecutor`
//     state
//         .db
//         .send(NewFsChangeLog {
//             name: name.into_inner(),
//         })
//         .from_err()
//         .and_then(|res| match res {
//             Ok(user) => Ok(HttpResponse::Ok().json(user)),
//             Err(_) => Ok(HttpResponse::InternalServerError().into()),
//         })
//         .responder()
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct MyUser {
//     name: String
// }

const MAX_SIZE: usize = 262_144; // max payload size is 256k

/// This handler manually load request payload and parse json object
fn index_add(
    (req, state): (HttpRequest<AppState>, State<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    // HttpRequest::payload() is stream of Bytes objects
    req.payload()
        // `Future::from_err` acts like `?` in that it coerces the error type from
        // the future into the final error type
        .from_err()
        // `fold` will asynchronously read each chunk of the request body and
        // call supplied closure, then it resolves to result of closure
        .fold(BytesMut::new(), move |mut body, chunk| {
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(::actix_web::error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        // `Future::and_then` can be used to merge an asynchronous workflow with a
        // synchronous workflow
        //
        // Douman NOTE:
        // The return value in this closure helps, to clarify result for compiler
        // as otherwise it cannot understand it
        .and_then(
            move |body| -> Box<Future<Item = HttpResponse, Error = Error>> {
                // body is loaded, now we can deserialize serde-json
                let r_fs_change_log_item = serde_json::from_slice::<NewFsChangeLog>(&body);

                // Send to the db for create
                match r_fs_change_log_item {
                    Ok(fs_change_log_item) => {
                        let res = state.db.send(fs_change_log_item).from_err()
                            .and_then(|res| match res {
                                Ok(fs_change_log_item) => Ok(HttpResponse::Ok().json(fs_change_log_item)),
                                Err(_) => Ok(HttpResponse::InternalServerError().into()),
                            });

                        Box::new(res)
                    }
                    Err(_) => Box::new(future::err(::actix_web::error::ErrorBadRequest("Json Decode Failed"))),
                }
            },
        )
}


fn add2(
    (item, state): (Json<NewFsChangeLog>, State<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    state
        .db
        .send(NewFsChangeLog {
            ..item.into_inner()
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("diesel-example");

    // Start 3 db executor actors
    // let manager = ConnectionManager::<SqliteConnection>::new("test.db");
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url).unwrap();

    let wd = std::env::var("WATCH_DIR").expect("WATCH_DIR must be set.");

    let db_addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));
    let db_addr1 = db_addr.clone();

    let wd_addr = Arbiter::start(move |ctx| {
        // use futures::stream::once;
        let dw = DirWatcher::new(&wd);
        WatcherDispatch::add_stream(dw, ctx);
        WatcherDispatch {
            app_state: AppState { db: db_addr1 }
        }
    });


    // watch("abc", AppState { db: addr.clone()}).unwrap();

    // Start http server
    server::new(move || {
        App::with_state(AppState { db: db_addr.clone() })
            // enable logger
            .middleware(middleware::Logger::default())
            // This can be called with:
            // curl -S --header "Content-Type: application/json" --request POST --data '{"name":"xyz"}'  http://127.0.0.1:8080/add
            // Use of the extractors makes some post conditions simpler such
            // as size limit protections and built in json validation.
            .resource("/add2", |r| {
                r.method(http::Method::POST)
                    .with_async_config(add2, |(json_cfg,)| {
                        json_cfg.0.limit(4096); // <- limit size of the payload
                    })
            })
            //  Manual parsing would allow custom error construction, use of
            //  other parsers *beside* json (for example CBOR, protobuf, xml), and allows
            //  an application to standardise on a single parser implementation.
            .resource("/add", |r| {
                r.method(http::Method::POST).with_async(index_add)
            })
        // .resource("/add/{name}", |r| r.method(http::Method::GET).with(add))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
