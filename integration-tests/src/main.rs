use futures_util::future::FutureExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ttb_backend::{config, handle_errors, oneshot, setup_store};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    email: String,
    password: String,
    role: String,
}

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    dotenv::dotenv().ok();

    let config = config::Config::new().expect("Config can't be set");

    let store = setup_store(&config).await?;
    store.client.database(&config.db_name).drop(None).await;

    let handler = oneshot(store).await;

    let user = User {
        email: "test@email.com".to_string(),
        password: "password".to_string(),
        role: "Admin".to_string(),
    };

    print!("Running register_new_user...");
    let result = std::panic::AssertUnwindSafe(register_new_user(&user))
        .catch_unwind()
        .await;

    match result {
        Ok(_) => println!("✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    let _ = handler.sender.send(1);
    Ok(())
}

async fn register_new_user(user: &User) {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/accounts")
        .json(&user)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await;

    assert_eq!(res.unwrap(), "Account added".to_string());
}
