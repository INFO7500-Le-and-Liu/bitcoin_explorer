use {
    dotenv,
    reqwest,
    tokio,
    crate::blocks_model::BlocksData,
};

const HOST_ROOT: &str = "https://blockchain.info";

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    // request function
    let client = reqwest::Client::new();

    client
        .get(url)
        .header("api_key", dotenv::var("API_KEY").expect("Cloud not find key: API_KEY"))
        .send()
        .await
        .expect("Failed to get response")
        .text()
        .await
        .expect("Failed to convert payload")
        
}

pub fn latest_blocks_request() -> BlocksData {
    let response = send_request(&[HOST_ROOT,"/latestblock"].join(""));
    serde_json::from_str(&response).expect("Failed to parse JSON")   
}

