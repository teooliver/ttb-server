use super::with_db;
use crate::controllers::seed;
use crate::db::DB;
use warp::Filter;

pub fn seed_clients(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("seed")
        .and(warp::get())
        .and(warp::path("clients"))
        .and(with_db(db.clone()))
        .and_then(seed::seed_clients)
}

pub fn seed_projects(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("seed")
        .and(warp::get())
        .and(warp::path("projects"))
        .and(with_db(db.clone()))
        .and_then(seed::seed_projects)
}

pub fn seed_tasks(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("seed")
        .and(warp::get())
        .and(warp::path("tasks"))
        .and(with_db(db.clone()))
        .and_then(seed::seed_tasks)
}

pub fn seed_all(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("seed")
        .and(warp::get())
        .and(warp::path("all"))
        .and(with_db(db.clone()))
        .and_then(seed::seed_all_data)
}

pub fn remove_all(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("seed")
        .and(warp::get())
        .and(warp::path("remove"))
        .and(with_db(db.clone()))
        .and_then(seed::remove_all_data)
}
