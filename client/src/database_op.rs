extern crate mysql;
// use dotenv::dotenv;
// use std::env;
use{ 
    crate::blocks_model::BlocksData, 
    crate::news_model::NewsData,
    dotenv::dotenv, 
    mysql::{*,prelude::*, Error as MySQLError, OptsBuilder, Pool, PooledConn}, 
    std::result::Result as StdResult,
    std::env
};

pub fn get_mysql_connection() -> StdResult<PooledConn, MySQLError> {
    // let builder = OptsBuilder::new()
    //     .ip_or_hostname(Some(dotenv::var("DB_HOSTNAME").expect("Failed to load hostname")))
    //     .user(Some(dotenv::var("DB_USERNAME").expect("Failed to load username")))
    //     .pass(Some(dotenv::var("DB_PASSWORD").expect("Failed to load password")));

    // let pool = Pool::new(builder)?;
    // let conn = pool.get_conn()?;
    // Ok(conn)

    let hostname = env::var("DB_HOSTNAME").unwrap_or_else(|_| "default_host".to_string());
    let username = env::var("DB_USERNAME").unwrap_or_else(|_| "default_user".to_string());
    let password = env::var("DB_PASSWORD").unwrap_or_else(|_| "default_password".to_string());

    let builder = OptsBuilder::new()
        .ip_or_hostname(Some(hostname))
        .user(Some(username))
        .pass(Some(password));

    let pool = Pool::new(builder)?;

    // Retrieve a connection from the pool
    let conn = pool.get_conn()?;
    println!("Successfully retrieved connection from pool."); // Debug message
    Ok(conn)
} 

pub fn create_database_and_table(conn: &mut PooledConn) -> StdResult<(), MySQLError> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();
    let database_name = env::var("DB_DATABASE").unwrap_or_else(|_| "bitcoin_blockchians".to_string());
    let block_table_name = env::var("DB_BLOCK_TABLE").unwrap_or_else(|_| "blocks".to_string());
    let news_table_name = env::var("DB_NEWS_TABLE").unwrap_or_else(|_| "news".to_string());
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


pub fn insert_block_data(conn: &mut PooledConn, block: &BlocksData) -> StdResult<(), MySQLError> {
    #[cfg(debug_assertions)]    
    dotenv().ok();
    let table_name = env::var("DB_BLOCK_TABLE").unwrap_or_else(|_| "blocks".to_string());

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
    #[cfg(debug_assertions)]   
    dotenv().ok();
    let table_name = env::var("DB_NEWS_TABLE").unwrap_or_else(|_| "news".to_string());
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
    println!("News Insert Done");
    Ok(())
}

