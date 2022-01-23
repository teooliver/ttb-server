pub mod clients;
pub mod projects;
pub mod seed;
pub mod tasks;

use crate::db::DB;
use std::convert::Infallible;

use warp::{http::StatusCode, Filter};

pub fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn health_check() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("health_check").map(|| StatusCode::OK)
}
