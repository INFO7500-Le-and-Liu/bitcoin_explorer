#[macro_use]
extern crate serde_derive;

mod blocks_model;
mod blocks_info;
mod init_database;

use {
    crate::blocks_model::BlocksData, init_database::{insert_data, init_database},
    // tokio::time::{sleep, Duration},
    std::thread::sleep,
    std::time::Duration,    
    // dotenv,
    // std::{io, thread, time},
};

fn main() {

    loop {
        let response : BlocksData= blocks_info::latest_blocks_request();
        println!("{:#?}", response);// debug

        // let mut conn:PooledConn =
        match init_database() {
            Ok(mut conn) => {
                println!("Database initialized successfully with connection");
                insert_data(&mut conn, response.height).expect("failed to insert data");
                conn
            } Err(e) => {
                eprintln!("Failed to initialize the database: {:#?}", e);
                return;
            }
        };
        sleep(Duration::from_secs(600));
    }
}

