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

use crate::{controllers::experiments, db::DB};

#[tokio::main]
async fn main() -> Result<()> {
    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "time_tracker_base=info,warp=debug".to_owned());

    let db = DB::init().await?;

    tracing_subscriber::fmt()
        // Determine which traces to record.
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
        .allow_methods(&[
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
        ]);

    let task_routes = routes::tasks::create_task(db.clone())
        .or(routes::tasks::get_tasks(db.clone()))
        .or(routes::tasks::fetch_task(db.clone()))
        .or(routes::tasks::fetch_tasks_grouped_by_date(db.clone()))
        .or(routes::tasks::edit_task(db.clone()))
        .or(routes::tasks::dangerously_delete_all_tasks(db.clone()))
        .or(routes::tasks::delete_task(db.clone()));

    let projects_routes = routes::projects::create_project(db.clone())
        .or(routes::projects::fetch_all_projects_grouped_by_client(
            db.clone(),
        ))
        .or(routes::projects::fetch_all_projects(db.clone()))
        .or(routes::projects::fetch_project(db.clone()))
        .or(routes::projects::dangerously_delete_all_projects(
            db.clone(),
        ))
        .or(routes::projects::delete_project(db.clone()));

    let client_routes = routes::clients::create_client(db.clone())
        .or(routes::clients::fetch_all_clients(db.clone()))
        .or(routes::clients::fetch_client(db.clone()))
        .or(routes::clients::delete_client(db.clone()));

    let seed_routes = routes::seed::seed_clients(db.clone())
        .or(routes::seed::seed_projects(db.clone()))
        .or(routes::seed::seed_tasks(db.clone()))
        .or(routes::seed::seed_all(db.clone()))
        .or(routes::seed::remove_all(db.clone()));

    let experiments = warp::path("experiments");

    let experiments_routes = experiments
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(warp::query::<experiments::PaginationQuery>())
        .and_then(experiments::pagination_with_query);

    // TODO: add "api/v1" to all routes path
    let routes = task_routes
        .or(projects_routes)
        .or(client_routes)
        .or(seed_routes)
        .or(experiments_routes)
        .or(routes::health_check())
        .with(cors)
        .with(warp::trace::request())
        .recover(error::handle_rejection);

    const PORT: u16 = 5000;
    println!("Started on port {PORT}");
    warp::serve(routes).run(([0, 0, 0, 0], PORT)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
