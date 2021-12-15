use crate::error::Error::*;
use crate::models::task::TaskGroupDates;
use crate::WebResult;
use crate::{db::DB, models::task::TaskRequest};
use serde::{self, Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationQuery {
    page: Option<u32>,
    size: Option<u32>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    previous: String, //Option
    next: String,     //Option
    total_pages: u32,
    total_items: i32,
    size: u32,
    start: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatinationResponse {
    results: Vec<TaskGroupDates>,
    pagination: Pagination,
}

pub async fn pagination_with_query(db: DB, query: PaginationQuery) -> WebResult<impl Reply> {
    const DEFAULT_PAGE: u32 = 1;
    const DEFAULT_LIMIT: u32 = 10;
    const START_PAGE: u8 = 1;

    let page = query.page.unwrap_or(DEFAULT_PAGE);
    let size = query.size.unwrap_or(DEFAULT_LIMIT);

    // if page == 0 {
    //     return reject::custom(PageError);
    //     // return Err(PageError);
    // }
    // if size == 0 {
    //     return reject::custom(LimitError);
    // }

    let tasks = db
        .get_tasks_grouped_by_date(Some(page), Some(size))
        .await
        .map_err(|e| reject::custom(e))?;

    // This doesnt look very elegant, Im sure there's a better way of doing this division.
    let total_pages = (tasks.total as f32 / size as f32).ceil() as u32;

    fn check_has_next_page(total: u32, current_page: u32) -> bool {
        if total == current_page {
            return false;
        }
        true
    }
    fn check_has_previous_page(current_page: u32) -> bool {
        if current_page == 1 {
            return false;
        }
        true
    }

    let has_next_page = check_has_next_page(total_pages, page);
    println!("Has NEXT page{}", has_next_page);
    let has_previous_page = check_has_previous_page(page);
    println!("Has PREVIOUS page {}", has_previous_page);

    let pagination = Pagination {
        previous: format!("/experiments?page={}&size={}", 3, 4),
        next: format!("/experiments?page={}&size={}", 3, 4),
        total_pages,
        total_items: tasks.total,
        size,
        start: START_PAGE as u32,
    };

    let result = PaginatinationResponse {
        results: tasks.result,
        pagination,
    };

    Ok(json(&result))
    // Ok(StatusCode::OK)
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
