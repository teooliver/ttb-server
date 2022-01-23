use reqwest::Error;

#[tokio::test]
async fn health_check_succeeds() -> Result<(), Error> {
    let res = reqwest::get("http://localhost:5000/health_check").await?;
    assert_eq!(res.status(), 200);
    Ok(())
}
