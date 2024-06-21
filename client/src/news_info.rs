use {
    crate::news_model::NewsResponse, 
    std::env, 
    reqwest, 
    tokio,
    dotenv::dotenv
};

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    // request function
    #[cfg(debug_assertions)]   
    dotenv().ok();
    let client = reqwest::Client::new();

    client
        .get(url)
        .header("api_key", env::var("CRYPTO_API_KEY").unwrap_or_else(|_| " ".to_string()) )
        .send()
        .await
        .expect("Failed to get response")
        .text()
        .await
        .expect("Failed to convert payload")
}

pub fn request_news() -> NewsResponse{
    let url =  "https://min-api.cryptocompare.com/data/v2/news/?lang=EN";
    let response: String = send_request(&url);
    // println!("{}",response);
    serde_json::from_str(&response).expect("Failed to parse JSON for news")
}