use crate::db::DB;
use crate::models::account::Account;
use warp::http::StatusCode;

pub async fn register(db: DB, account: Account) -> Result<impl warp::Reply, warp::Rejection> {
    match db.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
