// use clap::{App, Arg};
// use std::net::TcpListener;
// use file_transfer::ftp;
// use std::fs::File;
use tide::Request;
use tide::prelude::*;



#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8
}



#[async_std::main]
async fn main() -> tide::Result<()> {
    let port: &str = "127.0.0.1:8080";
    let mut app = tide::new();
    app.at("/orders/shoes").post(order_shoes);
    app.listen(port).await?;
    println!("Server listening at port {:?}", &port);
    Ok(())
}


async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}

