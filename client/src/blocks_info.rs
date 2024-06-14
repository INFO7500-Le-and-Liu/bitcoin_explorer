use {
    dotenv,
    reqwest,
    tokio,
    crate::blocks_model::{BlocksData,LatestBlock},
    std::thread::sleep,
    std::time::Duration,
};

const HOST_ROOT: &str = "https://blockchain.info";
const RETRY_LIMIT: usize = 3; 
const RETRY_DELAY: Duration = Duration::from_secs(2);

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    // request function
    let client = reqwest::Client::new();

    client
        .get(url)
        .header("api_key", dotenv::var("BLOCKCHAIN_API_KEY").expect("Cloud not find key: API_KEY"))
        .send()
        .await
        .expect("Failed to get response")
        .text()
        .await
        .expect("Failed to convert payload")
}

pub fn latest_blocks_request() -> LatestBlock {
    let response: String = send_request(&[HOST_ROOT,"/latestblock"].join(""));
    serde_json::from_str(&response).expect("Failed to parse JSON")   
}

pub fn request_block_by_height(&height: &usize) -> BlocksData {
    // print!("request block: {height}");
    let url = format!("{}/rawblock/{}", HOST_ROOT, height);
    // debug
    println!("request url: {url}");
    let mut attempts = 0;
    loop {
        attempts += 1;
        let response: String = send_request(&url);

        let truncated_string = if response.len() > 200 {
            &response[..20]
        } else {
            &response
        };
        println!("text content: {}", truncated_string);

        match serde_json::from_str(&response) {
            Ok(block_data) => return block_data,
            Err(e) => {
                eprintln!("Failed to parse JSON for block: {} and return value {} (attempt {}/{})", e, truncated_string, attempts, RETRY_LIMIT);
                if attempts >= RETRY_LIMIT {
                    panic!("Exceeded maximum retry attempts");
                }
                sleep(RETRY_DELAY);
            }
        }
    }
}

