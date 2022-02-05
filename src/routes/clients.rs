use super::with_db;
use crate::controllers::clients;
use crate::db::DB;
use warp::Filter;

pub fn create_client(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("clients")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(clients::create_client_handler)
}

pub fn fetch_all_clients(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("clients")
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(clients::fetch_all_clients_handler)
}

pub fn fetch_client(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("clients")
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(clients::fetch_client_handler)
}

pub fn delete_client(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("clients")
        .and(warp::delete())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(clients::delete_client_handler)
}
