use {
    // dotenv::dotenv,
    reqwest,
    // tokio,
    crate::blocks_model::{BlocksData,LatestBlock},
    std::thread::sleep,
    std::time::Duration,
    // std::env,
    reqwest::Error,
    reqwest::blocking::Client
};

const HOST_ROOT: &str = "https://blockchain.info";
const RETRY_LIMIT: usize = 3; 
const RETRY_DELAY: Duration = Duration::from_secs(2);


fn send_request(url: &str) -> Result<String, Error> {
    let client = Client::new();
    let response = client.get(url)
        .timeout(Duration::from_secs(100))
        .send()?
        .text()?;
    Ok(response)
}

pub fn latest_blocks_request() -> LatestBlock {
    let response: String = send_request(&[HOST_ROOT,"/latestblock"].join("")).expect("faile to request");
    serde_json::from_str(&response).expect("Failed to parse JSON")   
}


pub fn request_block_by_height(height: &usize) -> Result<BlocksData, String> {
    let url = format!("{}/rawblock/{}", HOST_ROOT, height);
    println!("Request URL: {}", url);

    let mut attempts = 0;

    loop {
        attempts += 1;

        match send_request(&url) {
            Ok(response) => {
                let truncated_string = if response.len() > 200 {
                    &response[..20]
                } else {
                    &response
                };
                println!("Text content: {}", truncated_string);

                match serde_json::from_str(&response) {
                    Ok(block_data) => return Ok(block_data),
                    Err(e) => {
                        eprintln!("Failed to parse JSON for block: {} and return value {} (attempt {}/{})", e, truncated_string, attempts, RETRY_LIMIT);
                        if attempts >= RETRY_LIMIT {
                            return Err(format!("Exceeded maximum retry attempts for height {}: {:?}", height, e));
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Request failed: {} (attempt {}/{})", e, attempts, RETRY_LIMIT);
                if attempts >= RETRY_LIMIT {
                    return Err(format!("Exceeded maximum retry attempts for height {}: {:?}", height, e));
                }
            }
        }

        sleep(RETRY_DELAY);
    }
}