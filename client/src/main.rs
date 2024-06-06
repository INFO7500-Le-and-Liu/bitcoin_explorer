#[macro_use]
extern crate serde_derive;

mod blocks_data;
mod blocks_info;
mod init_database;

use {
    crate::blocks_data::BlocksData, init_database::insert_data, mysql::PooledConn,
    // dotenv,
    // std::{io, thread, time},
};
use init_database::init_database;


fn main() {

    let response : BlocksData= blocks_info::latest_blocks_request();
    println!("{:#?}", response);// debug

    let mut conn:PooledConn = match init_database() {
        Ok(conn) => {
            println!("Database initialized successfully with connection");
            conn
        } Err(e) => {
            eprintln!("Failed to initialize the database: {:#?}", e);
            return;
        }
    };

    if let Err(e) = insert_data(&mut conn, response.height) {
        eprintln!("Error inserting data: {:?}", e);
        return; // end the program
    }

}

