use crate::db::DB;
use crate::models::account::Account;
use crate::WebResult;
use warp::http::StatusCode;
use warp::{reject, reply::json, Reply};

pub async fn register(account: Account, db: DB) -> WebResult<impl Reply> {
    match db.create_account(&account).await {
        Ok(_) => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Err(e) => {
            tracing::event!(tracing::Level::ERROR, "{:?}", e);
            Err(warp::reject::custom(e))
        }
    }
}
