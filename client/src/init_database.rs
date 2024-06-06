extern crate mysql;
use{ 
    mysql::{prelude::*, Error as MySQLError, OptsBuilder, Pool, PooledConn},
    std::result::Result as StdResult,
    dotenv,
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
    conn.query_drop("CREATE DATABASE IF NOT EXISTS bitcoin_blocks")?;
    conn.query_drop("USE bitcoin_blocks")?;
    conn.query_drop(
        "CREATE TABLE IF NOT EXISTS heights (
            height INT NOT NULL
        )",
    )?;
    Ok(())
}

pub fn insert_data(conn: &mut PooledConn, height: u64) -> StdResult<(), MySQLError> {
    let query = format!("INSERT INTO heights (height) VALUES ({})", height);
    conn.query_drop(query)?;
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
