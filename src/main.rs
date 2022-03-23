// use clap::{App, Arg};
// use std::net::TcpListener;
// use file_transfer::ftp;
// use std::fs::File;
use futures::executor::block_on;
use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

//todo read more on tcp

#[async_std::main]
async fn main() -> tide::Result<()> {
    let port: &str = "127.0.0.1:8080";
    let learn = learn_song();
    block_on(learn);
    let mut app = tide::new();
    tide::log::start();
    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.at("/orders/shoes").post(order_shoes);
    app.listen(port).await?;
    println!("Server listening at port {:?}", &port);

    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}

async fn learn_song() {
    println!("learn song")
}
async fn sing_song(song: &str) {
    println!("Singing {}", song)
}
async fn dance() {
    println!("Dancing to song")
}
