use async_std::channel::Receiver;
// use clap::{App, Arg};
// use std::net::TcpListener;
// use file_transfer::ftp;
// use std::fs::File;
use futures::executor::block_on;
// use futures::future::ok;
use std::io;
use tide::http::{Method, Request as OtherRequest, Response, StatusCode, Url};
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

    if role.len() >= 1 {
        let role = Some(role.trim());

        let chosen_role: &str = match role {
            Some(val) => val,
            None => {
                println!("Please choose a role, either you are a sender or a reciever");
                ""
            }
          
        };
        if chosen_role.trim() == "sender" || chosen_role.trim() == "s" {
            //client
            println!("Please enter reciever's port");
let mut receiver_port = String::from("");
io::stdin().read_line(&mut receiver_port).expect("Failed to provide a port");

if receiver_port.len() > 1 {
       let server_port = Some(receiver_port.trim());

        let chosen_port: &str = match server_port {
            Some(val) => val,
            None => {
                println!("Please choose a role, either you are a sender or a reciever");
                ""
            }
          
        };


             let client_port = format!("{}/hi",chosen_port);
        let mut res = surf::get(client_port).await?;
let string: String = res.body_string().await?;
        println!("{:?}", string);
}
        } else if chosen_role.trim() == "reciever" || chosen_role.trim() == "r" {
            //server
            let port: &str = "0.0.0.0:8080";
            let learn = learn_song();
            block_on(learn);
            let mut app = tide::new();
            tide::log::start();
            app.at("/").get(|_| async { Ok("Hello, world!") });
            app.at("/orders/shoes").post(order_shoes);
            app.at("/hi").post(some);
            app.at("/hi").get(|_| async {
                Ok("Hello there")
            });

               
            app.listen(port).await?;
            println!("Server listening at port {:?}", &port);
            

            
        } else {
            println!("type in a letter to start this process");
        }
        Ok(())
    } else {
        println!("type in a letter to start this process two");
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




async fn some(mut req: Request<()>) -> tide::Result {
  
    // let mut reqr = OtherRequest::new(Method::Post, Url::parse("http://127.0.0.1:8080/hi")?);
    // req.set_body("Hello, Nori!");
// dbg!(req);
// let AnyThing { any } =  req.body_json().await?;
    // let mut res = Response::new(StatusCode::Ok);
    // res.set_body("Hello, Chashu!");
    Ok(format!("jsut stuff {:?}", req).into())
}
