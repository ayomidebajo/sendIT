use clap::{App, Arg};
use std::net::TcpListener;
use file_transfer::ftp;
use std::fs::File;



fn main() {
    // let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
// for stream in listener.incoming() {
//     let stream = stream.unwrap();
//     println!("Connection established!")
// }

let ftpva = ftp::FTP::new("127.0.0.1:8080", File::create("example1.txt").unwrap());
    // Receives file from client
    ftpva.recv();

     let ftpse = ftp::FTP::new("127.0.0.1:8080", File::open("example.txt").unwrap());
    // Sends File
    ftpse.send();
// match listener.accept() {
//     Ok((_socket, addr)) => println!("client: {:?}", addr),
//     Err(e) => println!("couldn't get client: {:?}", e),
// }

//     let app = App::new("sendit")
// .about("An application for sending files")
// .arg(Arg::with_name("server").short('s').default_value()).get_matches();


    // let domain = app.value_of("server").unwrap();
    // println!("{:?}", domain);
}
