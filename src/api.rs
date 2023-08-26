use reqwest;
use reqwest::Error;

pub async fn get(url: &str) -> Result<(), Error> {
    let response = reqwest::get(url).await?;
    println!("Status: {}", response.status());
    let body = response.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}

pub async fn post(url: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .body("{\"title\":\"foo\", \"body\":\"bar\", \"userId\":1}")
        .send()
        .await?;
    println!("Status: {}", response.status());
    let body = response.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
