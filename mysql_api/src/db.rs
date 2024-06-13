extern crate mysql;
use{ 
    crate::model::{BlockData, NewsData}, 
    dotenv, 
    mysql::{prelude::*, Error as MySQLError, OptsBuilder, Pool}, 
    std::result::Result as StdResult
};

pub fn get_mysql_connection() -> StdResult<Pool, MySQLError> {
    let builder = OptsBuilder::new()
        .ip_or_hostname(Some(dotenv::var("DB_HOSTNAME").expect("Failed to load hostname")))
        .user(Some(dotenv::var("DB_USERNAME").expect("Failed to load username")))
        .pass(Some(dotenv::var("DB_PASSWORD").expect("Failed to load password")))
        .db_name(Some(dotenv::var("DB_DATABASE").expect("Failed to load database name")));
    // return a pool not a connection
    // rocket will handel the pool
    Ok(Pool::new(builder).expect("Failed to get Pool"))

} 

pub fn get_news(conn: &mut mysql::PooledConn) -> StdResult<Vec<NewsData>, MySQLError> {
    let news: Vec<NewsData> = conn.query_map(
        "SELECT id, title, url, body, source, tags FROM news",
        |(id, title, url, body, source, tags)| NewsData { 
            id, 
            title, 
            url, 
            body, 
            source, 
            tags
         },
    )?;
    Ok(news)
}

pub fn get_blocks(conn: &mut mysql::PooledConn) -> StdResult<Vec<BlockData>, MySQLError> {
    let blocks: Vec<BlockData> = conn.query_map(
        "SELECT hash, time, height, block_index, fee, n_tx FROM blocks",
        |(hash, time, height, block_index, fee, n_tx )| BlockData { 
            hash,
            time,
            height ,
            block_index,
            fee, 
            n_tx
        },
    )?;
    Ok(blocks)
}