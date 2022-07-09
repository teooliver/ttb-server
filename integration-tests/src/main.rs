use serde::{Deserialize, Serialize};
use serde_json::Value;
use ttb_backend::{config, handle_errors, oneshot, setup_store};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    email: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    dotenv::dotenv().ok();

    let config = config::Config::new().expect("Config can't be set");

    println!("CONFIG {:?}", config);
    let store = setup_store(&config).await?;
    let handler = oneshot(store).await;

    let user = User {
        email: "test@email.com".to_string(),
        password: "password".to_string(),
    };

    register_new_user(&user).await;

    // register_user();
    // login_user();
    // post_question();

    let _ = handler.sender.send(1);
    Ok(())
}

async fn register_new_user(user: &User) {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/registration")
        .json(&user)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await;

    assert_eq!(res.unwrap(), "Account added".to_string());
}
