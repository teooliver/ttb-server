use crate::WebResult;
use crate::{db::DB, models::project::ProjectRequest};
use tracing::{event, instrument, Level};
use warp::{http::StatusCode, reject, reply::json, Reply};

#[instrument]
pub async fn fetch_all_projects_grouped_by_client_handler(db: DB) -> WebResult<impl Reply> {
    event!(target: "practical_rust_book", Level::ERROR, "querying questions");
    // returns projects grouped by client
    let project = db
        .get_projects_grouped_by_client()
        .await
        .map_err(|e| reject::custom(e))?;

    event!(Level::INFO, db = "OK");
    Ok(json(&project))
}

pub async fn fetch_projects_handler(db: DB) -> WebResult<impl Reply> {
    let projects = db.get_all_projects().await.map_err(|e| reject::custom(e))?;
    Ok(json(&projects))
}

pub async fn fetch_project_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let project = db.find_project(&id).await.map_err(|e| reject::custom(e))?;
    Ok(json(&project))
}

pub async fn create_project_handler(body: ProjectRequest, db: DB) -> WebResult<impl Reply> {
    println!("BODY  {:?}", body);
    let project = db
        .create_project(&body)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json(&project))
}

pub async fn delete_project_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let deleted_id = db
        .delete_project(&id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json(&deleted_id))
}

pub async fn delete_all_projects_handler(db: DB) -> WebResult<impl Reply> {
    db.delete_all_projects()
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}
