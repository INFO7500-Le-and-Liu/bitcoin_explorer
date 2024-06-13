use {
    dotenv,
    reqwest,
    tokio,
    crate::news_model::{NewsData,NewsResponse}
};

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    // request function
    let client = reqwest::Client::new();

    client
        .get(url)
        .header("api_key", dotenv::var("CRYPTO_API_KEY").expect("Cloud not find key: API_KEY"))
        .send()
        .await
        .expect("Failed to get response")
        .text()
        .await
        .expect("Failed to convert payload")
}

pub fn request_news() -> NewsResponse{
    let url: &str = "https://min-api.cryptocompare.com/data/v2/news/?lang=EN";
    let response: String = send_request(url);
    // println!("{}",response);
    serde_json::from_str(&response).expect("Failed to parse JSON for news")
}