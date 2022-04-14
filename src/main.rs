// use clap::{App, Arg};
// use std::net::TcpListener;
// use file_transfer::ftp;
// use std::fs::File;
use futures::executor::block_on;
use futures::future::ok;
use std::io;
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
    println!("Please choose a role, you're either a sender or a reciever, type receiver or r to recieve, type sender or s to send");
    let mut role = String::from("");
    io::stdin()
        .read_line(&mut role)
        .expect("Failed to read line");

    let role = Some(role.trim());

    let chosen_role: &str = match role {
        Some(val) => val,
        _ => {
            println!("Please choose a role, either you are a sender or a reciever");
            ""
            //It complains it needs a char :D
            // break;
        }
    };
    if chosen_role.trim() == "sender" || chosen_role.trim() == "s" {
        println!("Implement logic for uploading!");
        Ok(())
    } else if chosen_role.trim() == "reciever" || chosen_role.trim() == "r" {
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
    } else {
        println!("type in a letter to start this process");
        Ok(())
    }
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
