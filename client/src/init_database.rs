extern crate mysql;
// use dotenv::dotenv;
// use std::env;
use{ 
    crate::blocks_model::BlocksData, 
    crate::news_model::NewsData,
    dotenv::dotenv, 
    mysql::{*,prelude::*, Error as MySQLError, OptsBuilder, Pool, PooledConn}, 
    std::result::Result as StdResult,

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
    let block_table_name = dotenv::var("DB_BLOCK_TABLE").expect("Failed to load table name");
    let news_table_name = dotenv::var("DB_NEWS_TABLE").expect("Failed to load table name");

    // database
    let s1 = format!("CREATE DATABASE IF NOT EXISTS {}",&database_name);
    let s2 = format!("USE {}",&database_name);
    // block table
    let s3 = format!("CREATE TABLE IF NOT EXISTS {} (
        hash TEXT,
        time TIMESTAMP ,
        block_index INTEGER ,
        height INTEGER PRIMARY KEY,
        fee INTEGER ,
        n_tx INTEGER 
        )",block_table_name);
    // news table
    let s4: String = format!("CREATE TABLE IF NOT EXISTS {} (
        id BIGINT PRIMARY KEY,
        title VARCHAR(255),
        url TEXT,
        body TEXT,
        source VARCHAR(255),
        tags VARCHAR(255)
        )",news_table_name);

    conn.query_drop(s1)?;
    conn.query_drop(s2)?;
    conn.query_drop(s3)?;
    conn.query_drop(s4)?;
    Ok(())
}

pub fn insert_block_data(conn: &mut PooledConn, block: &BlocksData) -> StdResult<(), MySQLError> {
    dotenv().ok();
    let table_name = dotenv::var("DB_BLOCK_TABLE").expect("Failed to load table name");

    // Convert Unix time to normal time using timestamp_opt
    let datetime = block.unix_time_to_datetime();
    let date_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();


    let insert_sql = format!("
    INSERT IGNORE INTO {} (hash, time, block_index, height, fee, n_tx)
    VALUES (\"{}\", \"{}\", {}, {}, {}, {})
    ", table_name, block.hash, date_str, block.block_index, block.height, block.fee, block.n_tx);
    // debug
    // println!("insert query:{}", insert_sql);

    conn.query_drop(insert_sql)?;

    println!("Insert Done");
    Ok(())
}

pub fn insert_news_data(conn: &mut PooledConn, news: &NewsData) -> StdResult<(), MySQLError> {
    dotenv().ok();
    let table_name = dotenv::var("DB_NEWS_TABLE").expect("Failed to load table name");
    let query = format!(
        "INSERT IGNORE INTO {} (id, title, url, body, source, tags) VALUES (:id, :title, :url, :body, :source, :tags)",
        table_name
    );
    let id: u64 = news.id.parse::<u64>().expect("Failed to parse news ID");

    conn.exec_drop(
        query,
        params! {
            "id" => id,
            "title" => &news.title,
            "url" => &news.url,
            "body" => &news.body,
            "source" => &news.source,
            "tags" => &news.tags,
        },
    )?;
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
