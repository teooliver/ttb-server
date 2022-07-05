use crate::models::task::TaskGroupDates;
use crate::utils::pagination::{
    has_next_page, has_previous_page, sanitize_pagination_query, Pagination, PaginationQuery,
};
use crate::WebResult;
use crate::{db::DB, models::task::TaskRequest};
use serde::{self, Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};

// TODO: remove before MR
// #[derive(Debug, Serialize, Deserialize)]
// pub struct PaginationQuery {
//     page: Option<u32>,
//     // TODO: change name to limit
//     size: Option<u32>,
// }

// TODO: remove before MR
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Pagination {
//     previous: Option<String>, //Option
//     next: Option<String>,     //Option
//     next_page: Option<u32>,
//     previous_page: Option<u32>,
//     total_pages: u32,
//     total_items: i32,
//     size: u32,
//     start: u32,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatinationResponse {
    results: Vec<TaskGroupDates>,
    pagination: Pagination,
}

pub async fn fetch_all_tasks_handler(db: DB) -> WebResult<impl Reply> {
    let tasks = db.get_all_tasks().await.map_err(|e| reject::custom(e))?;
    Ok(json(&tasks))
}

// TODO: FIX bug where not return proper Schema if there's no tasks found
pub async fn fetch_tasks_grouped_by_date_handler(
    db: DB,
    query: PaginationQuery,
) -> WebResult<impl Reply> {
    // TODO: remove before MR
    // const DEFAULT_PAGE: u32 = 1;
    // const DEFAULT_LIMIT: u32 = 2;
    // const START_PAGE: u32 = 1;

    // let page = query.page.unwrap_or(DEFAULT_PAGE);
    // let size = query.size.unwrap_or(DEFAULT_LIMIT);

    let sanitized_query = sanitize_pagination_query(query);
    let page = sanitized_query.page.unwrap();
    let size = sanitized_query.size.unwrap();

    let tasks = db
        .get_tasks_grouped_by_date(sanitized_query.page, sanitized_query.size)
        .await
        .map_err(|e| reject::custom(e))?;

    // This doesnt look very elegant, Im sure there's a better way of doing this division.
    let total_pages = (tasks.total as f32 / size as f32).ceil() as u32;

    let has_next_page = has_next_page(total_pages, page);
    let has_previous_page = has_previous_page(page);

    let pagination = Pagination {
        previous: if has_previous_page {
            Some(format!("/tasks/group?page={}&size={}", page - 1, size))
        } else {
            None
        },
        next: if has_next_page {
            Some(format!("/tasks/group?page={}&size={}", page + 1, size))
        } else {
            None
        },
        next_page: if has_next_page { Some(page + 1) } else { None },
        previous_page: if has_previous_page {
            Some(page - 1)
        } else {
            None
        },
        total_pages,
        total_items: tasks.total,
        size,
        // start: START_PAGE,
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
