use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockData {
    pub height: usize,    //height in whole chain
}