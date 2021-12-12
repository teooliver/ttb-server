use crate::WebResult;
use crate::{db::DB, models::task::TaskRequest};
use serde::{self, Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Debug, Serialize, Deserialize)]
pub struct FooQuery {
    page: Option<String>,
    size: Option<String>,
}

pub async fn pagination_with_query(db: DB, query: FooQuery) -> WebResult<impl Reply> {
    println!("hwekjhrwek");

    if !query.page.is_none() {
        println!("Page {:?}", query.page);
    }
    if !query.size.is_none() {
        println!("Size {:?}", query.size);
    }

    let _tasks = db
        .get_tasks_grouped_by_date()
        .await
        .map_err(|e| reject::custom(e))?;
    // Ok(json(&tasks))
    Ok(StatusCode::OK)
}
