#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct BlocksData {
    pub hash: String, // hash ID
    pub time: u64,  //Unix time as timestamp
    pub block_index: u64,   //index in whole chain
    pub height: u64,    //height in whole chain
    //txIndexes: Vec<u64>,    //transactions included in the block
}