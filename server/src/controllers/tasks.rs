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
    previous: Option<String>, //Option
    next: Option<String>,     //Option
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

pub async fn fetch_all_tasks_handler(db: DB) -> WebResult<impl Reply> {
    let tasks = db.get_all_tasks().await.map_err(|e| reject::custom(e))?;
    Ok(json(&tasks))
}

pub async fn fetch_tasks_grouped_by_date(db: DB, query: PaginationQuery) -> WebResult<impl Reply> {
    const DEFAULT_PAGE: u32 = 1;
    const DEFAULT_LIMIT: u32 = 7;
    const START_PAGE: u8 = 1;

    let page = query.page.unwrap_or(DEFAULT_PAGE);
    let size = query.size.unwrap_or(DEFAULT_LIMIT);

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
    let has_previous_page = check_has_previous_page(page);

    let pagination = Pagination {
        previous: if has_previous_page {
            Some(format!("/experiments?page={}&size={}", page - 1, size))
        } else {
            None
        },
        next: if has_next_page {
            Some(format!("/experiments?page={}&size={}", page + 1, size))
        } else {
            None
        },
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
