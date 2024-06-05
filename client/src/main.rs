#[macro_use]
extern crate serde_derive;

mod blocks_data;
mod blocks_info;

use {
    crate::blocks_data::BlocksData,
    dotenv,
    std::{io, thread, time},
};


fn main() {
    let response = blocks_info::latest_blocks_request();
    println!("{:#?}", response);
}

