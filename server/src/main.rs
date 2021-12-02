#[allow(dead_code)]
mod controllers;
mod db;
mod error;
mod models;

use std::convert::Infallible;
use warp::{hyper::Method, Filter, Rejection};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

use crate::{
    controllers::{clients, projects, seed, tasks},
    db::DB,
};

#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            // "User-Agent",
            // "Sec-Fetch-Mode",
            // "Referer",
            // "Origin",
            // "Access-Control-Request-Method",
            // "Access-Control-Request-Headers",
            "content-type",
        ])
        // .allow_methods(vec!["POST", "GET"]);
        .allow_methods(&[
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
        ]);
    // .allow_header("content-type");
    // .allow_headers(["application/json", "content-type"]);
    // .allow_credentials(true);
    // .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);

    // TODO: add "api/v1" to all routes
    let tasks = warp::path("tasks");

    let task_routes = tasks
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(tasks::create_task_handler)
        .or(tasks
            .and(warp::get())
            .and(warp::path::end())
            .and(with_db(db.clone()))
            .and_then(tasks::fetch_all_tasks_handler))
        .or(tasks
            .and(warp::get())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(tasks::fetch_task_handler))
        .or(tasks
            .and(warp::get())
            .and(warp::path("group"))
            .and(with_db(db.clone()))
            .and_then(tasks::fetch_tasks_grouped_by_date))
        .or(tasks
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(tasks::edit_task_handler))
        .or(tasks
            .and(warp::delete())
            .and(warp::path("dangerously-delete-all-tasks"))
            .and(with_db(db.clone()))
            .and_then(tasks::delete_all_tasks_handler))
        .or(tasks
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(tasks::delete_task_handler));

    let projects = warp::path("projects");

    let projects_routes = projects
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(projects::create_project_handler)
        .or(projects
            .and(warp::get())
            .and(warp::path::end())
            .and(with_db(db.clone()))
            .and_then(projects::fetch_all_projects_handler))
        .or(projects
            .and(warp::get())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(projects::fetch_project_handler))
        .or(projects
            .and(warp::delete())
            .and(warp::path("dangerously-delete-all-projects"))
            .and(with_db(db.clone()))
            .and_then(projects::delete_all_projects_handler))
        .or(projects
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(projects::delete_project_handler));

    let clients = warp::path("clients");
    let client_routes = clients
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(clients::create_client_handler)
        .or(clients
            .and(warp::path::end())
            .and(with_db(db.clone()))
            .and_then(clients::fetch_all_clients_handler))
        .or(clients
            .and(warp::get())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(clients::fetch_client_handler))
        .or(clients
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(clients::delete_client_handler));

    let seed = warp::path("seed");

    let seed_routes = seed
        .and(warp::get())
        .and(warp::path("clients"))
        .and(with_db(db.clone()))
        .and_then(seed::seed_clients)
        .or(seed
            .and(warp::get())
            .and(warp::path("projects"))
            .and(with_db(db.clone()))
            .and_then(seed::seed_projects))
        .or(seed
            .and(warp::get())
            .and(warp::path("tasks"))
            .and(with_db(db.clone()))
            .and_then(seed::seed_tasks))
        .or(seed
            .and(warp::get())
            .and(warp::path("all"))
            .and(with_db(db.clone()))
            .and_then(seed::seed_all_data))
        .or(seed
            .and(warp::get())
            .and(warp::path("remove"))
            .and(with_db(db.clone()))
            .and_then(seed::remove_all_data));

    let routes = task_routes
        .or(projects_routes)
        .or(client_routes)
        .or(seed_routes)
        .with(cors)
        .recover(error::handle_rejection);

    println!("Started on port 5000");
    warp::serve(routes).run(([0, 0, 0, 0], 5000)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
