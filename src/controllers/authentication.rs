use crate::db::DB;
use crate::types::account::{Account, NewAccount, Session};
use crate::{handle_errors, WebResult};
use argon2::{self, Config};
use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use rand::Rng;
use std::{env, future};
use warp::http::StatusCode;
use warp::{Filter, Reply};

pub async fn register(account: NewAccount, db: DB) -> WebResult<impl Reply> {
    let hashed_password = hash(account.password.as_bytes());

    let account = NewAccount {
        email: account.email,
        password: hashed_password,
        role: account.role,
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
    let key = env::var("PASETO_KEY").unwrap();

    let current_date_time = Utc::now();
    let dt = current_date_time + chrono::Duration::days(1);

    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from(key.as_bytes()))
        .set_expiration(&dt)
        .set_not_before(&Utc::now())
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to construct paseto token w/ builder!")
}

pub fn verify_token(token: String) -> Result<Session, handle_errors::Error> {
    let key = env::var("PASETO_KEY").unwrap();

    let token = paseto::tokens::validate_local_token(
        &token,
        None,
        &key.as_bytes(),
        &paseto::tokens::TimeBackend::Chrono,
    )
    .map_err(|_| handle_errors::Error::CannotDecryptToken)?;
    serde_json::from_value::<Session>(token).map_err(|_| handle_errors::Error::CannotDecryptToken)
}

pub fn auth() -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization").and_then(|token: String| {
        let token = match verify_token(token) {
            Ok(t) => t,
            Err(_) => return future::ready(Err(warp::reject::reject())),
        };
        future::ready(Ok(token))
    })
}

#[cfg(test)]
mod authentication_tests {
    use super::{auth, env, issue_token, ObjectId};
    use crate::handle_errors::Error::InvalidIDError;

    #[tokio::test]
    async fn post_questions_auth() {
        env::set_var("PASETO_KEY", "RANDOM WORDS WINTER MACINTOSH PC");

        let account_id = ObjectId::parse_str("62c074351967d4641049ef88".to_string())
            .map_err(|_| InvalidIDError("hello".to_owned()))
            .unwrap();

        let token = issue_token(account_id);
        let filter = auth();
        let res = warp::test::request()
            .header("Authorization", token)
            .filter(&filter);

        assert_eq!(res.await.unwrap().account_id, account_id);
    }
}
