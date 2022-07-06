use crate::args::Args;
use crate::directory;
use ansi_term::Colour::Green;
use atty::Stream;
use curl::easy::Easy;
use ignore::{WalkBuilder, WalkState};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{io, io::Read, process};

#[derive(Debug)]
pub struct PathPrinter<'a> {
    pub path: String,
    pub reg_exp: &'a Regex,
    port_addr: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileSearch {
    pub file_bytes: Vec<u8>,
    pub file_name: String,
}

impl FileSearch {
    pub fn _new(file_bytes: &[u8], file_name: String) -> FileSearch {
        FileSearch {
            file_bytes: file_bytes.to_vec(),
            file_name,
        }
    }
}

impl<'a> PathPrinter<'a> {
    pub fn new(path: String, reg_exp: &Regex, port_addr: String) -> PathPrinter {
        PathPrinter {
            path,
            reg_exp,
            port_addr,
        }
    }

    pub fn print(&self) {
        if atty::isnt(Stream::Stdout) {
            self.print_to_non_tty();
        } else {
            self.print_to_tty();
        }
    }

    fn print_path(&self) -> String {
        //  implement a error handler
        // Todo Handle error
        send_file_post(&self.path, &self.port_addr);
        self.path.to_string()
    }

    fn print_to_non_tty(&self) {
        println!("{}", self.path);
    }

    fn print_to_tty(&self) {
        match self.reg_exp.find(&self.path) {
            Some(result) => {
                let matched_str = &self.path[result.start()..result.end()];

                let colored_match = Green.bold().paint(matched_str).to_string();
                let _path = self.path.replace(matched_str, &colored_match);

                let _ = &self.print_path();
            }

            None => {
                println!("{}", self.path);
            }
        }
    }
}

fn truncate_working_dir_path(path: String, working_dir_path: &str) -> String {
    if path.contains(&working_dir_path) {
        path.replace(&working_dir_path, ".")
    } else {
        path.clone()
    }
}
fn is_match(reg_exp: &Regex, maybe_exclude_reg_exp: &Option<Regex>, path: &str) -> bool {
    let is_path_matched = reg_exp.is_match(path);

    match maybe_exclude_reg_exp {
        Some(exclude_reg_exp) => is_path_matched && !exclude_reg_exp.is_match(path),
        None => is_path_matched,
    }
}

#[derive(Debug)]
pub struct Walker<'a> {
    args: &'a Args,
}

impl<'a> Walker<'a> {
    pub fn new(args: &Args) -> Walker {
        Walker { args: args }
    }

    //Todo return a string so that we can use the path
    pub fn print_file_path(&self) {
        use std::sync::mpsc;
        use std::thread;

        let thread_num: usize = 10;
        let max_depth: usize = 2;

        let walker = WalkBuilder::new(&self.args.root_path)
            .hidden(true)
            .git_ignore(true)
            .max_depth(Some(max_depth))
            .threads(thread_num)
            .build_parallel();

        let (tx, rx) = mpsc::channel::<String>();
        let reg_exp = self.args.reg_exp.clone();
        let port = self.args.port_address.clone();

        let print_thread = thread::spawn(move || -> io::Result<()> {
            for path in rx.iter() {
                PathPrinter::new(path, &reg_exp, String::from(&port)).print();
            }
            Ok(())
        });
        // println!("tx {:?}", print_thread);

        let working_dir_path = directory::working_dir_path();
        walker.run(|| {
            let tx = tx.clone();
            let reg_exp = self.args.reg_exp.clone();
            let exclude_directories = self.args.exclude_directories.clone();
            let maybe_exclude_reg_exp = self.args.exclude_reg_exp.clone();
            let working_dir_path = working_dir_path.clone();

            Box::new(move |path_entry| {
                // println!("{:?}", path_entry);
                if let Ok(entry) = path_entry {
                    if exclude_directories && !entry.path().is_file() {
                        WalkState::Continue
                    } else {
                        let path = entry.path().display().to_string();
                        let path = truncate_working_dir_path(path, &working_dir_path);

                        if is_match(&reg_exp, &maybe_exclude_reg_exp, &path) {
                            match tx.send(path) {
                                Ok(_) => WalkState::Continue,
                                Err(_) => WalkState::Quit,
                            }
                        } else {
                            WalkState::Continue
                        }
                    }
                } else {
                    WalkState::Continue
                }
            })
        });

        drop(tx);

        if let Err(err) = print_thread.join().unwrap() {
            if err.kind() != io::ErrorKind::BrokenPipe {
                if let Some(err_msg) = err.into() {
                    eprintln!("{}", err_msg);
                }

                process::exit(1);
            }
        }
    }
}

// It takes in a file path.
fn send_file_post(file_from_arg: &str, port_addr: &str) -> tide::Result {
    // initialise
    let mut easy = Easy::new();
    let port = format!("{}/hi", port_addr);
    easy.url(&port).unwrap();

    // reads file path
    let file = std::fs::read(file_from_arg)?;

    //extracts name of the file from file path
    let (.., file_name) = file_from_arg
        .rsplit_once(std::path::MAIN_SEPARATOR)
        .unwrap();

    // creates the necessary type to send the file in bytes
    let new_post = FileSearch {
        file_name: file_name.to_string(),
        file_bytes: file,
    };

    // Unwrap into a vector, which can be likened to bytes
    let send_file_req_body = serde_json::to_vec(&new_post).unwrap();

    // make and send request
    easy.post(true).unwrap();
    easy.post_field_size(send_file_req_body.len() as u64)
        .unwrap();

    let mut transfer = easy.transfer();
    transfer
        .read_function(|buf| Ok(send_file_req_body.as_slice().read(buf).unwrap_or(0)))
        .unwrap();
    transfer.perform().unwrap();

    Ok(format!("okay sent!").into())
}
