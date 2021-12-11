use crate::WebResult;
use crate::{db::DB, models::task::TaskRequest};
use warp::{http::StatusCode, reject, reply::json, Reply};

pub async fn fetch_all_tasks_handler(db: DB) -> WebResult<impl Reply> {
    let tasks = db.get_all_tasks().await.map_err(|e| reject::custom(e))?;
    Ok(json(&tasks))
}

pub async fn fetch_tasks_grouped_by_date(db: DB) -> WebResult<impl Reply> {
    let tasks = db
        .get_tasks_grouped_by_date()
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json(&tasks))
}

pub async fn fetch_task_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let tasks = db.find_task(&id).await.map_err(|e| reject::custom(e))?;
    Ok(json(&tasks))
}

pub async fn create_task_handler(body: TaskRequest, db: DB) -> WebResult<impl Reply> {
    let task = db.create_task(&body).await.map_err(|e| reject::custom(e))?;
    // TODO: Return the created object
    // This is returning the id of the inserted object
    Ok(json(&task))
}

pub async fn delete_all_tasks_handler(db: DB) -> WebResult<impl Reply> {
    db.delete_all_tasks().await.map_err(|e| reject::custom(e))?;
    // TODO: Return the deleted object
    Ok(StatusCode::OK)
}

pub async fn edit_task_handler(id: String, body: TaskRequest, db: DB) -> WebResult<impl Reply> {
    println!("GOT HERE Inside Task");
    db.edit_task(&id, &body)
        .await
        .map_err(|e| reject::custom(e))?;
    // TODO: Return the edited object
    Ok(StatusCode::OK)
}
pub async fn delete_task_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let taks_id = db.delete_task(&id).await.map_err(|e| reject::custom(e))?;
    // Return the deleted object
    Ok(json(&taks_id))
}
