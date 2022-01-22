#[allow(dead_code)]
// #![warn(clippy::all)]
mod controllers;
mod db;
mod error;
mod models;
mod routes;

use std::convert::Infallible;
use warp::{hyper::Method, Filter, Rejection};

use tracing_subscriber::fmt::format::FmtSpan;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

// use crate::routes::with_db;
use crate::{
    controllers::{clients, experiments, projects, seed, tasks},
    db::DB,
};

#[tokio::main]
async fn main() -> Result<()> {
    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "time_tracker_base=info,warp=debug".to_owned());

    let db = DB::init().await?;

    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

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
    // let tasks = warp::path("tasks");

    let task_routes = routes::tasks::create_task(db.clone())
        .or(routes::tasks::get_tasks(db.clone()))
        .or(routes::tasks::fetch_task(db.clone()))
        .or(routes::tasks::fetch_tasks_grouped_by_date(db.clone()))
        .or(routes::tasks::edit_task(db.clone()))
        .or(routes::tasks::delete_all_tasks(db.clone()))
        .or(routes::tasks::delete_task(db.clone()));

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
            .and_then(projects::fetch_all_projects_handler)
            .with(warp::trace(|info| {
                tracing::info_span!(
                    "fetch_all_projects_handler request",
                    method = %info.method(),
                    path = %info.path(),
                    id = %uuid::Uuid::new_v4(),
                )
            })))
        .or(projects
            .and(warp::get())
            .and(warp::path("all"))
            .and(with_db(db.clone()))
            .and_then(projects::fetch_projects_handler))
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

    let experiments = warp::path("experiments");

    let experiments_routes = experiments
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(warp::query::<experiments::PaginationQuery>())
        .and_then(experiments::pagination_with_query);

    let routes = task_routes
        .or(projects_routes)
        .or(client_routes)
        .or(seed_routes)
        .or(experiments_routes)
        .with(cors)
        .with(warp::trace::request())
        .recover(error::handle_rejection);

    println!("Started on port 5000");
    warp::serve(routes).run(([0, 0, 0, 0], 5000)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
