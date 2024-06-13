#[macro_use]
extern crate serde_derive;

mod blocks_model;
mod blocks_info;
mod init_database;

use {
    crate::blocks_model::BlocksData, 
    blocks_model::LatestBlock, 
    init_database::{init_database, insert_data}, 
    std::{thread::sleep, time::Duration}    
    // dotenv,
    // std::{io, thread, time},
};

fn main() {

    let mut height = 847680;
    loop {
       
        let response : BlocksData = blocks_info::request_block_by_height(&height);
        let latest : LatestBlock = blocks_info::latest_blocks_request();

        println!("{:#?}", response);// debug
        println!("{:#?}", latest);

        // let block_time = response.unix_time_to_datetime();
        // println!("----{}",block_time);
        // let block_time = block_time.format("%Y-%m-%d %H:%M:%S").to_string();
        // println!("{}",block_time);

        if latest.block_index == response.block_index {
            println!("sleeping...");
            sleep(Duration::from_secs(1200));
            continue;
        }

        // let mut conn:PooledConn =
        match init_database() {
            Ok(mut conn) => {
                println!("Database initialized successfully with connection");
                insert_data(&mut conn, &response).expect("failed to insert data");
                conn
            } Err(e) => {
                eprintln!("Failed to initialize the database: {:#?}", e);
                return;
            }
        };
        height += 1;
        sleep(Duration::from_secs(10));
    }
}

