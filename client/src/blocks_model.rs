// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "lowercase")]
use chrono::{DateTime, Utc};
use std::time::SystemTime;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct BlocksData {
    pub hash: String, // hash ID
    pub time: i64,  //Unix time as timestamp
    pub block_index: usize,   //index in whole chain
    pub height: usize,    //height in whole chain
    pub fee: usize,     // operation fee for the miner
    pub n_tx: usize,    //transcations num
    // pub transcation: Vec<TransAction>,
}
impl BlocksData {
    pub fn unix_time_to_datetime(&self) -> DateTime<Utc> {
        // Unix时间戳是从1970年1月1日开始计算的秒数或者毫秒数
        // 这里我们假设它是秒数
        let duration = std::time::Duration::from_secs(self.time as u64);
        // 将duration转换为SystemTime
        let system_time = SystemTime::UNIX_EPOCH.checked_add(duration).unwrap();
        // 将SystemTime转换为DateTime<Utc>
        DateTime::<Utc>::from(system_time)
    }
}

// #[derive(Deserialize, Debug)]
// pub struct TransAction {
//     pub value: u64,
// }

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct LatestBlock {
    pub height: usize,
    pub block_index: usize,
}