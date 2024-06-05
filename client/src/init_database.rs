extern crate mysql;
use mysql::{Pool, PooledConn, OptsBuilder, Error as MySQLError};
use std::result::Result as StdResult;

pub fn get_mysql_connection() -> StdResult<PooledConn, MySQLError> {
    let builder = OptsBuilder::new()
        .ip_or_hostname(Some("localhost"))
        .user(Some("root"))
        .pass(Some("12345678"))
        .db_name(Some("demo"));

    let pool = Pool::new(builder)?;
    let conn = pool.get_conn()?;
    Ok(conn)
} 
