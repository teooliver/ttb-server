use super::with_db;
use crate::controllers::authentication;
use crate::db::DB;
use warp::Filter;

pub fn create_account(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("accounts")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(authentication::register)
}

// TODO: refactor path to `accounts/login`
pub fn fetch_account(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(authentication::login)
}
