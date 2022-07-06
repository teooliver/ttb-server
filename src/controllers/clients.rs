use crate::db::DB;
use crate::types::client::ClientRequest;
use crate::WebResult;
use warp::{reject, reply::json, Reply};

pub async fn fetch_all_clients_handler(db: DB) -> WebResult<impl Reply> {
    // let tasks = db.get_all_clients().await.map_err(|e| reject::custom(e))?;
    // Ok(json(&tasks))

    // =============== Better? ==================;
    match db.get_all_clients().await {
        Ok(tasks) => Ok(warp::reply::json(&tasks)),
        Err(e) => {
            tracing::event!(tracing::Level::ERROR, "{:?}", e);
            Err(warp::reject::custom(e))
        }
    }
}

pub async fn fetch_client_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let tasks = db.find_client(&id).await.map_err(|e| reject::custom(e))?;
    Ok(json(&tasks))
}

pub async fn create_client_handler(body: ClientRequest, db: DB) -> WebResult<impl Reply> {
    let client = db
        .create_client(&body)
        .await
        .map_err(|e| reject::custom(e))?;

    Ok(json(&client))
}

pub async fn delete_client_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let deleted_id = db.delete_client(&id).await.map_err(|e| reject::custom(e))?;

    Ok(json(&deleted_id))
}
