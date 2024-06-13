use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockData {
    pub hash: String, // hash ID
    pub time: String,  //Unix time as timestamp
    pub block_index: usize,   //index in whole chain
    pub height: usize,    //height in whole chain
    pub fee: usize,     // operation fee for the miner
    pub n_tx: usize,    //transcations num
    // pub transcation: Vec<TransAction>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewsData {
    pub id: String,
    pub title: String,
    pub url: String,
    pub body: String,
    pub source: String,
    pub tags: String
}