mod model;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use futures::executor::block_on;
use ignore::{Walk, WalkBuilder};
// use std::env;
// use std::fs;
// use std::fs::File;
use std::io;
use std::io::{stdout, Read, Write};
// use std::io::prelude::*;
// use std::io::BufReader;
use curl::easy::Easy;
// use http_types::{Method, Request as OtherRequest, Response, StatusCode, Url};
use tide::prelude::*;
use tide::{Request, Response, StatusCode};

type ItemsDb = Arc<Mutex<HashMap<usize, model::File<'static>>>>;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[derive(Debug, Deserialize)]
struct Car<'a> {
    color: &'a str,
}

pub enum Error {
    RewquestError
}
//todo figure out how to connect this server to a database

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("Please choose a role, you're either a sender or a reciever, type receiver or r to recieve, type sender or s to send port");

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
            //CLIENT
            // port http://192.168.100.23:8080/
                  println!("Please enter reciever's port");
            let mut receiver_port = String::from("");
            io::stdin()
                .read_line(&mut receiver_port)
                .expect("Failed to provide a port");

            if receiver_port.len() > 1 {
                let server_port = Some(receiver_port.trim());

                let chosen_port: &str = match server_port {
                    Some(val) => val,
                    None => {
                        println!("Please choose a role, either you are a sender or a reciever");
                        ""
                    }
                };
                let mut easy = Easy::new();
            easy.url(chosen_port).unwrap();
      
                println!("Please enter file name to send, type -c followed by the filename to send a file, -g to get all sent files");

                loop {
                    let mut action_client = String::from("");
                    io::stdin()
                        .read_line(&mut action_client)
                        .expect("Failed to read line");
                    println!("Acion! {}", action_client);

                    match action_client.trim() {
                        "-c" => dance(),
                        "-g" => {
                            sing_song();
                        }
                        _ => {
                            println!("invalid command")
                        }
                    }
                }
            }
        } else if chosen_role.trim() == "reciever" || chosen_role.trim() == "r" {
            //SERVER
            let port: &str = "0.0.0.0:8080";
            let learn = learn_song();
            block_on(learn);

            //SERVER initiation
            let mut app = tide::new();
            tide::log::start();
            app.at("/").get(|_| async { Ok("Hello demola, world!") });
            app.at("/orders/shoes").post(order_shoes);
            app.at("/hi").post(some);
            app.at("/hi").get(|_| async { Ok("Hello there") });

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

async fn test_post(mut req: Request<()>) -> tide::Result {
    let post = req.body_json().await?;
    println!("{:?}", post);
    Ok(format!("Hello, {:?}!", post).into())
}
async fn learn_song() {
    println!("learn song")
}
fn sing_song() -> tide::Result {
    let mut data_to_upload = &b"hello world"[..];
    let mut handle = Easy::new();
    handle.url("http://192.168.100.23:8080").unwrap();

    handle
        .write_function(|data| {
            stdout().write_all(data).unwrap();
            Ok(data.len())
        })
        .unwrap();
    println!(" oh sing {:?}", handle.perform().unwrap());

    Ok(format!("jsut stuff").into())
}
fn dance() {
    let mut easy = Easy::new();
    easy.url("http://192.168.100.23:8080/hi").unwrap();
    // println!("Please enter reciever's port");
    easy.post_fields_copy(&b"hello world"[..]).unwrap();

    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();

    println!(" oh hi{:?}", easy.perform().unwrap());
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

// pub type CustomResult = ;

pub async fn get_shopping_list_items(items_db: ItemsDb) ->  Result<tide::ResponseBuilder, Error>{
    let local_db = items_db.lock().await;
    // let local_db: Vec<model::File<'static>> = local_db.values().cloned().collect();
    Ok(Response::builder(200).body(json!({ "any": "Into<Body>"})))
}