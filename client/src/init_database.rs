extern crate mysql;
// use dotenv::dotenv;
// use std::env;
use{ 
    crate::blocks_model::BlocksData, dotenv::dotenv, mysql::{prelude::*, Error as MySQLError, OptsBuilder, Pool, PooledConn}, std::result::Result as StdResult
};

pub fn get_mysql_connection() -> StdResult<PooledConn, MySQLError> {
    let builder = OptsBuilder::new()
        .ip_or_hostname(Some(dotenv::var("DB_HOSTNAME").expect("Failed to load hostname")))
        .user(Some(dotenv::var("DB_USERNAME").expect("Failed to load username")))
        .pass(Some(dotenv::var("DB_PASSWORD").expect("Failed to load password")));

    let pool = Pool::new(builder)?;
    let conn = pool.get_conn()?;
    Ok(conn)
} 

pub fn create_database_and_table(conn: &mut PooledConn) -> StdResult<(), MySQLError> {
    dotenv().ok();
    let database_name = dotenv::var("DB_DATABASE").expect("Failed to load database name");
    let table_name = dotenv::var("DB_TABLE").expect("Failed to load table name");

    let s1 = format!("CREATE DATABASE IF NOT EXISTS {}",&database_name);
    let s2 = format!("USE {}",&database_name);
    let s3 = format!("CREATE TABLE IF NOT EXISTS {} (
        hash TEXT,
        time TIMESTAMP ,
        block_index INTEGER ,
        height INTEGER PRIMARY KEY,
        fee INTEGER ,
        n_tx INTEGER 
        )",table_name);

    conn.query_drop(s1)?;
    conn.query_drop(s2)?;
    conn.query_drop(s3)?;
    Ok(())
}

pub fn insert_data(conn: &mut PooledConn, block: &BlocksData) -> StdResult<(), MySQLError> {
    dotenv().ok();
    let table_name = dotenv::var("DB_TABLE").expect("Failed to load table name");

    // Convert Unix time to normal time using timestamp_opt
    let datetime = block.unix_time_to_datetime();
    let date_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();


    let insert_sql = format!("
    INSERT INTO {} (hash, time, block_index, height, fee, n_tx)
    VALUES (\"{}\", \"{}\", {}, {}, {}, {})
    ", table_name, block.hash, date_str, block.block_index, block.height, block.fee, block.n_tx);

    println!("insert query:{}", insert_sql);

    conn.query_drop(insert_sql)?;

    println!("Insert Done");
    Ok(())
}

// init the database
pub fn init_database() -> StdResult<PooledConn, MySQLError> {
    let mut conn = match get_mysql_connection() {
        Ok(conn) => {
            println!("Successfully connected to the database!");
            conn
        }
        Err(e) => {
            eprintln!("Failed to connect to the database: {:?}", e);
            return Err(e);
        }
    };

    if let Err(e) = create_database_and_table(&mut conn) {
        eprintln!("Error creating database or table: {:?}", e);
        return Err(e);
    }
    Ok(conn)
}
