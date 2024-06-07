extern crate mysql;
use{ 
    mysql::{prelude::*, Error as MySQLError, OptsBuilder, Pool, PooledConn},
    std::result::Result as StdResult,
    crate::model::BlockData,
    dotenv,
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

pub fn get_blocks(conn: &mut mysql::PooledConn) -> StdResult<Vec<BlockData>, MySQLError> {
    let blocks: Vec<BlockData> = conn.query_map(
        "SELECT height FROM blocks",
        |height| BlockData { height },
    )?;
    Ok(blocks)
}