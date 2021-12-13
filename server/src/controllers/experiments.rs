use crate::WebResult;
use crate::{db::DB, models::task::TaskRequest};
use serde::{self, Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationQuery {
    page: Option<u32>,
    size: Option<u32>,
}

pub async fn pagination_with_query(db: DB, query: PaginationQuery) -> WebResult<impl Reply> {
    // Should query have defaults values?

    println!("hwekjhrwek");

    let tasks = db
        .get_tasks_grouped_by_date(query.page, query.size)
        .await
        .map_err(|e| reject::custom(e))?;

    if !query.page.is_none() {
        println!("Page {:?}", query.page);
        // let slice = &tasks[0..2];
        // return Ok(json(slice));
    }
    if !query.size.is_none() {
        println!("Size {:?}", query.size);
    }

    // response should be something like to tasks:
    // results: &tasks,
    // "paging":  {
    //    "previous":  "ksdjhfkdjfk/kjhfksd/ksdjhfsk",
    //    "next":  "ksdjhfkdjfk/kjhfksd/ksdjhfsk",
    //    "total_pages": 20,
    //    "total_items": 100000;
    //    "size": 5,
    //    "start": 0
    //}
    // "_links": {
    //     "base": "/tasks",
    //     "context": "",
    //     "next": "/rest/api/tasks/page?limit=5&start=5",
    //     "self": "/page"
    // },
    // "limit": 5,
    // "size": 5,
    // "start": 0

    // refs: https://developer.atlassian.com/server/confluence/pagination-in-the-rest-api/

    Ok(json(&tasks))
    // Ok(StatusCode::OK)
}
