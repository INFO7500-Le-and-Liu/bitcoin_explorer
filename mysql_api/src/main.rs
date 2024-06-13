#[macro_use] extern crate rocket;

use rocket::serde::json::Json;

use rocket::figment::Figment;

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
    let pool = get_mysql_connection().expect("Failed to get MySQL connection pool");
    let mut conn = pool.get_conn().expect("Failed to get connection from pool");

    match get_blocks(&mut conn) {
        Ok(blocks) => Json(blocks),
        Err(_) => Json(vec![]), // return empty if error
    }
}

#[get("/news")]
fn get_news_handler() -> Json<Vec<NewsData>> {
    let pool = get_mysql_connection().expect("Failed to get MySQL connection pool");
    let mut conn = pool.get_conn().expect("Failed to get connection from pool");

    match get_news(&mut conn) {
        Ok(news) => Json(news),
        Err(_) => Json(vec![]), // return empty if error
    }
}

#[launch]
fn rocket() -> _ {
    let figment = Figment::from(rocket::Config::default())
        .merge(("port", 8080)); // Set port to 8080

    rocket::custom(figment)
        .mount("/", routes![get_blocks_handler, get_news_handler])
}