mod config;
#[allow(dead_code)]
mod controllers;
mod db;
mod routes;
mod types;
mod utils;

use handle_errors;

use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use std::convert::Infallible;
use warp::{hyper::Method, Filter, Rejection};

use tracing_subscriber::fmt::format::FmtSpan;

type Result<T> = std::result::Result<T, handle_errors::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

use crate::{controllers::experiments, db::DB};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let config = config::Config::new().expect("Config can't be set");

    let log_filter = format!(
        "handle_errors={},rust_web_dev={},warp={}",
        config.log_level, config.log_level, config.log_level
    );

    let db = DB::init(
        &format!(
            "mongodb://{}:{}/{}",
            config.db_host, config.db_port, config.db_name
        ),
        config.db_name,
    )
    .await?;

    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! {"email": 1})
        .options(options)
        .build();

    db.get_accounts_collection()
        .create_index(model, None)
        .await
        .expect("error creating index!");

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
        .allow_methods(&[
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
        ]);

    // Move these routes to their own files. I.e: routes/tasks ?
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

    let account_routes = routes::authentication::create_account(db.clone())
        .or(routes::authentication::fetch_account(db.clone()));

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
        .or(account_routes)
        .or(routes::health_check())
        .with(cors)
        .with(warp::trace::request())
        .recover(handle_errors::handle_rejection);

    // tracing::info!("Q&A service build ID {}", env!("RUST_WEB_DEV_VERSION"));

    println!("Started on port {}", config.port);
    warp::serve(routes).run(([0, 0, 0, 0], config.port)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
