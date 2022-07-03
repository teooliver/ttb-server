use crate::db::DB;
use crate::models::account::Account;
use crate::{handle_errors, WebResult};
use argon2::{self, Config};
use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use rand::Rng;
use warp::http::StatusCode;
use warp::{reject, reply::json, Reply};

pub async fn register(account: Account, db: DB) -> WebResult<impl Reply> {
    let hashed_password = hash(account.password.as_bytes());

    let account = Account {
        email: account.email,
        password: hashed_password,
        _id: None,
        first_name: None,
        last_name: None,
        created_at: None,
        updated_at: None,
    };

    match db.create_account(&account).await {
        Ok(_) => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Err(e) => {
            // Add propper error message here
            tracing::event!(tracing::Level::ERROR, "{:?}", e);
            Err(warp::reject::custom(e))
        }
    }
}

pub fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub async fn login(login: Account, db: DB) -> WebResult<impl Reply> {
    // let login = db
    //     .get_account(&email)
    //     .await
    //     .map_err(|e| reject::custom(e))?;

    // Ok(json(&login))

    match db.get_account(&login.email).await {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(
                        account._id.expect("id not found"),
                    )))
                } else {
                    Err(warp::reject::custom(handle_errors::Error::WrongPassword))
                }
            }
            Err(e) => Err(warp::reject::custom(
                handle_errors::Error::ArgonLibraryError(e),
            )),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

fn issue_token(account_id: ObjectId) -> String {
    let current_date_time = Utc::now();
    let dt = current_date_time + chrono::Duration::days(1);
    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from("RANDOM WORDS WINTER MACINTOSH PC".as_bytes()))
        .set_expiration(&dt)
        .set_not_before(&Utc::now())
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to construct paseto token w/ builder!")
}
