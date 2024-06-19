#[macro_use]
extern crate serde_derive;

mod blocks_model;
mod blocks_info;
mod news_info;
mod database_op;
mod news_model;


use {
    crate::blocks_model::BlocksData, blocks_model::LatestBlock, database_op::{init_database, insert_block_data,insert_news_data}, std::{thread::sleep, time::Duration}    
    // dotenv,
    // std::{io, thread, time},
};

fn main() {


    let latest_block : LatestBlock = blocks_info::latest_blocks_request();

    let mut height = latest_block.height - 10;
    println!("get latest ten blocks from {}", height);


    let mut timer = 0;
    sleep(Duration::from_secs(10));
    loop {
        // if one block take 10 mins, 144 blocks roughly take 1 day
        // 24 * 60 / 10 = 144
        if timer % 144 == 0 { 
            timer /= 144;
            let news:news_model::NewsResponse  = news_info::request_news();
            println!("news id:{:#?}", news.data[1].id);
        
            match init_database() {
                Ok(mut conn) => {
                    println!("Database initialized successfully with connection");
                    for i in 0..20 {
                        insert_news_data(&mut conn, &news.data[i]).expect("failed to insert data");
                    }
                    conn
                }
                Err(e) => {
                    eprintln!("Failed to initialize the database: {:#?}", e);
                    return;
                }
            };
        }
        // println!("request block: {} ...",height);
        let response : BlocksData = blocks_info::request_block_by_height(&height);
        sleep(Duration::from_secs(10));
        println!("request latest block ...");
        let latest : LatestBlock = blocks_info::latest_blocks_request();
        
        // debug
        println!("block index:{:#?}", response.block_index);
        println!("LATEST index:{:#?}", latest.block_index);

        if response.block_index == latest.block_index - 1 {
            println!("sleeping...");
            sleep(Duration::from_secs(1200));
            continue;
        }            
            
        match init_database() {
            Ok(mut conn) => {
                println!("Database initialized successfully with connection");
                insert_block_data(&mut conn, &response).expect("failed to insert data");
                conn
            } Err(e) => {
                eprintln!("Failed to initialize the database: {:#?}", e);
                return;
            }
        };
        height += 1;
        timer += 1;
        println!("---------next round----------");//debug
        sleep(Duration::from_secs(10));
       
    }
}

