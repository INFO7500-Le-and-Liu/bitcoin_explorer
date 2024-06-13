#[macro_use]
extern crate serde_derive;

mod blocks_model;
mod blocks_info;
mod news_info;
mod init_database;
mod news_model;


use {
    crate::blocks_model::BlocksData, blocks_model::LatestBlock, init_database::{init_database, insert_data}, news_model::NewsData, std::{thread::sleep, time::Duration}    
    // dotenv,
    // std::{io, thread, time},
};

fn main() {

    let mut height = 847680;
    loop {
        let news:news_model::NewsResponse  = news_info::request_news();
        println!("{:#?}", news.Data);
       
        let response : BlocksData = blocks_info::request_block_by_height(&height);
        let latest : LatestBlock = blocks_info::latest_blocks_request();
        
        // debug
        println!("{:#?}", response.block_index);
        println!("{:#?}", latest.block_index);
        
        if latest.block_index == response.block_index - 1 {
            println!("sleeping...");
            sleep(Duration::from_secs(1200));
            continue;
        }

            
            
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

