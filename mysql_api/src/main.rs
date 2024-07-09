#[macro_use] extern crate rocket;
extern crate serde_json;


use rocket::serde::json::Json;
use rocket::figment::Figment;
use rocket_cors::{CorsOptions, AllowedOrigins};


mod db;
mod model;
use {
    db::{ get_blocks, get_mysql_connection, get_news},
    model::{BlockData, NewsData},
};

#[get("/")]
fn index() -> String {
    // let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    // format!("Application running on port: {}", port)
    format!("Hello World")
}

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
    let port:u32 = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap();
    println!("read the heroku port:{}", port);

    let figment = Figment::from(rocket::Config::default())
        .merge(("address", "0.0.0.0"))
        .merge(("port", port)); // Set port to 8080

    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://frontend:4200", // allow port
        "http://localhost:4200",
        "http://0.0.0.0:4200",
        "https://bitcoinexplorefront1-44442e571a5a.herokuapp.com/",
    ]);
    
    let cors = CorsOptions {
        allowed_origins,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS fairing");

    rocket::custom(figment)
    .mount("/", routes![index,get_blocks_handler, get_news_handler])
    .attach(cors) // add CORS config
}