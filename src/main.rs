mod args;
mod model;
use futures::executor::block_on;
use std::process;
use std::{collections::HashMap, sync::Arc};
use std::{fs, io};
use tide::prelude::*;
use tide::{Request, Response};
use tokio::sync::Mutex;

mod directory;
mod search_and_print;

type ItemsDb = Arc<Mutex<HashMap<usize, model::File<'static>>>>;

#[derive(Debug, Deserialize)]

// test struct to test out requests
struct Animal {
    name: String,
    legs: u8,
}

#[derive(Debug, Deserialize)]
pub enum Error {
    RewquestError,
}

// #[derive(Debug)]
// struct FileName {
//     filename: String,
// }

//todo figure out how to connect this server to a database

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("Please choose a role, you're either a sender or a reciever, type receiver or r to recieve, type sender or s to send and the name of the file you want to send");

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

            println!("Type -c to send the file, and -q to exit");

            loop {
                let mut action_client = String::from("");
                io::stdin()
                    .read_line(&mut action_client)
                    .expect("Failed to read line");
                // println!("Acion! {}", action_client);

                match action_client.trim() {
                    "-c" => send_file(),
                    "-q" => process::exit(1),
                    _ => {
                        println!("invalid command");
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
    // testing and practice=ing how to unwrap a request
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}

async fn learn_song() {
    println!("learn song")
}

fn send_file() {
    let new_arg = args::Args::parse();

    let _prat = search_and_print::Walker::new(&new_arg).print_file_path();
    // println!("Args, {:?}", new_arg.filename);
    // let file = FileName {
    //     filename: new_arg.filename.to_string(),
    // };
}

async fn some(mut req: Request<()>) -> tide::Result {
    let file_size = req.body_bytes().await?;

    println!("printing filesize {:?}", file_size);

    let file: search_and_print::FileSearch = serde_json::from_slice(&*file_size).unwrap();
    println!("heehe 2 {:?}", &file_size);
    fs::write(file.file_name, file.file_bytes).expect("unable to write file");

    Ok(format!("jst stuff {:?}", req).into())
}

// test functions feel free to edit this to test out the functionalities
pub async fn get_shopping_list_items(_items_db: ItemsDb) -> Result<tide::ResponseBuilder, Error> {
    Ok(Response::builder(200).body(json!({ "any": "Into<Body>"})))
}
