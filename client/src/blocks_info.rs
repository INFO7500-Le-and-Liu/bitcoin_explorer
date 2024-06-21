use {
    dotenv::dotenv,
    reqwest,
    tokio,
    crate::blocks_model::{BlocksData,LatestBlock},
    std::thread::sleep,
    std::time::Duration,
    std::env,
    reqwest::Error,
    reqwest::blocking::Client
};

const HOST_ROOT: &str = "https://blockchain.info";
const RETRY_LIMIT: usize = 3; 
const RETRY_DELAY: Duration = Duration::from_secs(2);

// #[tokio::main]
// pub async fn send_request(url: &str) -> String {
//     // request function
//     #[cfg(debug_assertions)]   
//     dotenv().ok();
//     let client = reqwest::Client::new();

//     client
//         .get(url)
//         .header("api_key", env::var("BLOCKCHAIN_API_KEY").unwrap_or_else(|_| " ".to_string()) )
//         .send()
//         .await
//         .expect("Failed to get response")
//         .text()
//         .await
//         .expect("Failed to convert payload")
// }

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

// pub fn request_block_by_height(&height: &usize) -> BlocksData {
//     // print!("request block: {height}");
//     let url = format!("{}/rawblock/{}", HOST_ROOT, height);
//     // debug
//     println!("request url: {url}");
//     let mut attempts = 0;
//     loop {
//         attempts += 1;
//         let response: String = send_request(&url);

//         let truncated_string = if response.len() > 200 {
//             &response[..20]
//         } else {
//             &response
//         };
//         println!("text content: {}", truncated_string);

//         match serde_json::from_str(&response) {
//             Ok(block_data) => return block_data,
//             Err(e) => {
//                 eprintln!("Failed to parse JSON for block: {} and return value {} (attempt {}/{})", e, truncated_string, attempts, RETRY_LIMIT);
//                 if attempts >= RETRY_LIMIT {
//                     panic!("Exceeded maximum retry attempts");
//                 }
//                 sleep(RETRY_DELAY);
//             }
//         }
//     }
// }


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