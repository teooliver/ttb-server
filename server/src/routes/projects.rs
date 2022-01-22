use super::with_db;
use crate::controllers::projects;
use crate::db::DB;
use warp::Filter;

pub fn create_project(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("projects")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(projects::create_project_handler)
}

pub fn fetch_all_projects_grouped_by_client(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("projects")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(projects::fetch_all_projects_grouped_by_client_handler)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "fetch_all_projects_grouped_by_client_handler request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }))
}

pub fn fetch_all_projects(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("projects")
        .and(warp::get())
        .and(warp::path("all"))
        .and(with_db(db.clone()))
        .and_then(projects::fetch_projects_handler)
}

pub fn fetch_project(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("projects")
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(projects::fetch_project_handler)
}

pub fn dangerously_delete_all_projects(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("projects")
        .and(warp::delete())
        .and(warp::path("dangerously-delete-all-projects"))
        .and(with_db(db.clone()))
        .and_then(projects::delete_all_projects_handler)
}

pub fn delete_project(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("projects")
        .and(warp::delete())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(projects::delete_project_handler)
}
