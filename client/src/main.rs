#[macro_use]
extern crate serde_derive;

mod blocks_data;
mod blocks_info;
mod init_database;

// use {
//     crate::blocks_data::BlocksData,
//     dotenv,
//     std::{io, thread, time},
// };
use init_database::get_mysql_connection;


fn main() {
    match get_mysql_connection() {
        Ok(_conn) => {
            println!("Successfully connected to the database!");
            // operation
            
        }
        Err(e) => {
            eprintln!("Failed to connect to the database: {:?}", e);
        }
    }



    //let response : BlocksData= blocks_info::latest_blocks_request();
    // println!("{:#?}", response);
}

