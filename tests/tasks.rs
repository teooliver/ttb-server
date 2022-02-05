// #[test]
// fn it_adds_two() {
//     assert_eq!(4, 4);
// }

// #[test]
// fn it_adds_two() {
//     assert_eq!(4, adder::add_two(2));
// }

// mod adder {
//     pub fn add_two(int: i8) -> i8 {
//         int + 2
//     }
// }

use reqwest::{self, Error};

#[tokio::test]
async fn test_get_tasks() -> Result<(), Error> {
    let res = reqwest::get("http://localhost:5000/tasks").await?;
    assert_eq!(res.status(), 200);
    Ok(())
}

#[tokio::test]
async fn delete_all_tasks() -> Result<(), Error> {
    let client = reqwest::Client::new();

    let res = client
        .delete("http://localhost:5000/tasks/dangerously-delete-all-tasks")
        .send()
        .await?;

    assert_eq!(res.status(), 200);
    Ok(())
}
