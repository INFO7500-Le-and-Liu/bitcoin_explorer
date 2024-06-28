extern crate mysql;
use{ 
    crate::model::{BlockData, NewsData},  
    mysql::{prelude::*, Error as MySQLError, OptsBuilder, Pool,PooledConn},  
    std::result::Result as StdResult,
    std::env,
    dotenv::dotenv
};

// pub fn get_mysql_connection() -> StdResult<Pool, MySQLError> {
//     // local environment
//     // let builder = OptsBuilder::new()
//     //     .ip_or_hostname(Some(dotenv::var("DB_HOSTNAME").expect("Failed to load hostname")))
//     //     .user(Some(dotenv::var("DB_USERNAME").expect("Failed to load username")))
//     //     .pass(Some(dotenv::var("DB_PASSWORD").expect("Failed to load password")))
//     //     .db_name(Some(dotenv::var("DB_DATABASE").expect("Failed to load database name")));
//     // // return a pool not a connection
//     // // rocket will handel the pool
//     // println!("get the pools success"); // debug
//     // Ok(Pool::new(builder).expect("Failed to get Pool"))

//     #[cfg(debug_assertions)]    
//     dotenv().ok();

//     // CI/CD 
//     let hostname = env::var("DB_HOSTNAME").unwrap_or_else(|_| "default_host".to_string());
//     let username = env::var("DB_USERNAME").unwrap_or_else(|_| "default_user".to_string());
//     let password = env::var("DB_PASSWORD").unwrap_or_else(|_| "default_password".to_string());
//     let database = env::var("DB_DATABASE").unwrap_or_else(|_| "default_db".to_string());

//     let builder = OptsBuilder::new()
//         .ip_or_hostname(Some(hostname))
//         .user(Some(username))
//         .pass(Some(password))
//         .db_name(Some(database));

//     // Attempt to create a connection pool
//     let pool = Pool::new(builder)?;

//     println!("Successfully created connection pool."); // Debug message
//     Ok(pool)    
// } 


pub fn get_mysql_connection() -> StdResult<PooledConn, MySQLError> {
    // let builder = OptsBuilder::new()
    //     .ip_or_hostname(Some(dotenv::var("DB_HOSTNAME").expect("Failed to load hostname")))
    //     .user(Some(dotenv::var("DB_USERNAME").expect("Failed to load username")))
    //     .pass(Some(dotenv::var("DB_PASSWORD").expect("Failed to load password")));

    // let pool = Pool::new(builder)?;
    // let conn = pool.get_conn()?;
    // Ok(conn)
    #[cfg(debug_assertions)]    
    dotenv().ok();
    let hostname = env::var("DB_HOSTNAME").unwrap_or_else(|_| "f2fbe0zvg9j8p9ng.cbetxkdyhwsb.us-east-1.rds.amazonaws.com".to_string());
    let username = env::var("DB_USERNAME").unwrap_or_else(|_| "r4xiw6ghgvuy7ozf".to_string());
    let password = env::var("DB_PASSWORD").unwrap_or_else(|_| "tk7s231xgov5fblm".to_string());
    let database_name = env::var("DB_DATABASE").unwrap_or_else(|_| "if6on175le9kpi29".to_string());

    println!("{} {} {} {}", hostname, username, password, database_name);

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


pub fn get_news(conn: &mut mysql::PooledConn) -> StdResult<Vec<NewsData>, MySQLError> {
    // Obtain the database name from the environment or use a default
    let database_name = env::var("DB_DATABASE").unwrap_or_else(|_| "if6on175le9kpi29".to_string());

    // Select the database
    conn.query_drop(format!("USE {}", database_name))
        .expect("Failed to select database");



    let table_name = env::var("DB_NEWS_TABLE").unwrap_or_else(|_| "news".to_string());
    let query = format!("SELECT id, title, url, body, source, tags FROM {} ORDER BY id DESC",table_name);
    
    match conn.query_map(
        query,
        |(id, title, url, body, source, tags)| NewsData { 
            id, 
            title, 
            url, 
            body, 
            source, 
            tags
         },
    ) {
        Ok(news) => {
            //println!("Fetched news: {:?}", news);
            println!("get news OK");
            Ok(news)
        },
        Err(e) => {
            eprintln!("Failed to fetch news: {:?}", e);
            Err(e)
        }
    }

}

pub fn get_blocks(conn: &mut mysql::PooledConn) -> StdResult<Vec<BlockData>, MySQLError> {
    // Obtain the database name from the environment or use a default
    let database_name = env::var("DB_DATABASE").unwrap_or_else(|_| "if6on175le9kpi29".to_string());

    // Select the database
    conn.query_drop(format!("USE {}", database_name))
        .expect("Failed to select database");

    let table_name = env::var("DB_BLOCK_TABLE").unwrap_or_else(|_| "blocks".to_string());
    let query = format!("SELECT hash, time, height, block_index, fee, n_tx FROM {} ORDER BY height DESC",table_name);

    match conn.query_map(
        query,
        |(hash, time, height, block_index, fee, n_tx )| BlockData { 
            hash,
            time,
            height ,
            block_index,
            fee, 
            n_tx
        },
        
    ){
        Ok(blocks) => {
            //println!("Fetched news: {:?}", blocks);
            println!("get blocks OK");
            Ok(blocks)
        },
        Err(e) => {
            eprintln!("Failed to fetch news: {:?}", e);
            Err(e)
        }
    }
}
