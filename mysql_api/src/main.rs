#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::figment::Figment;
use rocket_cors::{CorsOptions, AllowedOrigins};
use std::io::{self, Write};
// use rocket::figment::{Figment, providers::{Format, Toml, Env}};
// use mysql::Pool;


mod db;
mod model;
use {
    db::{ get_blocks, get_mysql_connection, get_news},
    model::{BlockData, NewsData},
};

#[get("/blocks")]
fn get_blocks_handler() -> Json<Vec<BlockData>> {
    // let pool = get_mysql_connection().expect("Failed to get MySQL connection pool");
    // let mut conn = pool.get_conn().expect("Failed to get connection from pool");

    let mut conn = get_mysql_connection().expect("failed to connect");

    match get_blocks(&mut conn) {
        Ok(blocks) => Json(blocks),
        Err(_) => Json(vec![]), // return empty if error
    }
}

#[get("/news")]
fn get_news_handler() -> Json<Vec<NewsData>> {
    // let pool = get_mysql_connection().expect("Failed to get MySQL connection pool");
    // let mut conn = pool.get_conn().expect("Failed to get connection from pool");

    let mut conn = get_mysql_connection().expect("failed to connect");

    match get_news(&mut conn) {
        Ok(news) => Json(news),
        Err(_) => Json(vec![]), // return empty if error
    }
}

#[launch]
fn rocket() -> _ {
    let port: u32 = std::env::var("PORT").expect("PORT environment variable not set").parse().expect("Invalid PORT");
    println!("Using port: {}", port);
    io::stdout().flush().unwrap();

    //----------testing -----------
    let hostname = std::env::var("DB_HOSTNAME").unwrap_or_else(|_| "default_host".to_string());
    let username = std::env::var("DB_USERNAME").unwrap_or_else(|_| "default_user".to_string());
    let password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| "default_password".to_string());
    println!("env var testing: {} {} {}", hostname, username, password);


    // let port:u32 = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap();
    // println!("read the heroku port:{}", port);

    let figment = Figment::from(rocket::Config::default())
        .merge(("address", "0.0.0.0"))
        .merge(("port", port)); // Set port to 8080

    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://frontend:4200", // allow port
        "http://localhost:4200",
        "http://0.0.0.0:4200",
    ]);
    
    let cors = CorsOptions {
        allowed_origins,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS fairing");

    rocket::custom(figment)
    .mount("/", routes![get_blocks_handler, get_news_handler])
    .attach(cors) // add CORS config
}