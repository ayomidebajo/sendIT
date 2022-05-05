mod play;

use async_std::channel::Receiver;
use futures::executor::block_on;
use ignore::{Walk, WalkBuilder};
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use http_types::{Method, Request as OtherRequest, Response, StatusCode, Url};
use tide::new;
use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[derive(Debug, Deserialize)]
struct Car<'a> {
    color: &'a str,
}

struct Action {
    post: String,
get: String
}
//todo read more on tcp

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
                println!("Please enter file name to send, type -c followed by the filename to send a file, -g to get all sent files");
                // let path = env::current_dir()?;
                // println!("The current directory is {}", path.display());

                // To list files in current directory
                // for result in Walk::new("./") {
                //     // Each item yielded by the iterator is either a directory entry or an
                //     // error, so either print the path or the error.
                //     match result {
                //         Ok(entry) => println!("{}", entry.path().display()),
                //         Err(err) => println!("ERROR: {}", err),
                //     }
                // }
                

                loop {
let mut action_client =  String::from("");
io::stdin().read_line(&mut action_client).expect("Failed to read line");
println!("Acion! {}", action_client);

//  match action_client.trim() {
//     Some(val) => val,
//     _ => println!("stuff")
    
// }


match action_client.trim() {
  "-c" => {
    let mut req=  OtherRequest::new(Method::Get, Url::parse("http://192.168.100.23:8080/hi").unwrap());
    req.set_body("Hello, Nori!");
    req.body_json().await?
    },
    // req.set_body("Hello, Nori!");},
  "-g" => {println!("getting stuff done")},
  _ => {println!("just there")}
}
                    // let f = File::open("example1.txt")?;
                    // let mut buf_reader = BufReader::new(f);
                    // let mut contents = String::new();

                    // buf_reader.read_to_string(&mut contents)?;
                    // println!("meta {:?}", contents);

                    // let client_port = format!("{}", chosen_port);
                    // println!("server port {}", client_port);
                    // let mut res = surf::get(client_port).await?;
                    // let string: String = res.body_string().await?;
                    // println!("response {:?}", string);
                }
                // LOGIC TEST FOR SENDING FILES
                // for entry in fs::read_dir(".")? {
                //         let dir = entry?;
                //         println!("{:?}", dir.path());
                //     }
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

        // for entry in fs::read_dir(".")? {
        //         let dir = entry?;
        //         println!(" uhm {:?}", dir.path());
        //     }

        //  let Ok(entries) = fs::read_dir(".");
        //     for entry in entries {
        //         if let Ok(entry) = entry {
        //             // Here, `entry` is a `DirEntry`.
        //             println!("{:?}", entry.file_name());
        //         }
        //     }
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
