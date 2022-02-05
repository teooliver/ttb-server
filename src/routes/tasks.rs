use super::with_db;
use crate::controllers::tasks;
use crate::db::DB;
use warp::Filter;

/// POST /tasks
pub fn create_task(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("tasks")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(tasks::create_task_handler)
}

/// GET /tasks
pub fn get_tasks(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("tasks")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(tasks::fetch_all_tasks_handler)
}

/// GET /tasks/:id
pub fn fetch_task(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("tasks")
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(tasks::fetch_task_handler)
}

/// GET /tasks/group/
/// GET /tasks/group?size=20&page=1
pub fn fetch_tasks_grouped_by_date(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("tasks")
        .and(warp::get())
        .and(warp::path("group"))
        .and(with_db(db.clone()))
        .and(warp::query::<tasks::PaginationQuery>())
        .and_then(tasks::fetch_tasks_grouped_by_date_handler)
}

/// PUT /tasks/:id
pub fn edit_task(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("tasks")
        .and(warp::put())
        .and(warp::path::param())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(tasks::edit_task_handler)
}

/// DELETE /tasks/dangerously-delete-all-tasks
pub fn dangerously_delete_all_tasks(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("tasks")
        .and(warp::delete())
        .and(warp::path("dangerously-delete-all-tasks"))
        .and(with_db(db.clone()))
        .and_then(tasks::delete_all_tasks_handler)
}

/// DELETE /tasks/:id
pub fn delete_task(
    db: DB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("tasks")
        .and(warp::delete())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(tasks::delete_task_handler)
}
