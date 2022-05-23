use ansi_term::Colour::Green;
use atty::Stream;
use ignore::{ WalkBuilder, WalkState};
use regex::Regex;
use std::{io, process};

use crate::directory;
use crate::Args::Args;

#[derive(Debug)]
pub struct PathPrinter<'a> {
    path: String,
    reg_exp: &'a Regex,
}

impl<'a> PathPrinter<'a> {
    pub fn new(path: String, reg_exp: &Regex) -> PathPrinter {
        PathPrinter { path, reg_exp }
    }

    pub fn print(&self) {
        // println!(" print! {:?}", self);
        if atty::isnt(Stream::Stdout) {
            self.print_to_non_tty();
        } else {
            self.print_to_tty();
        }
    }

    fn print_to_non_tty(&self) {
        println!("{}", self.path);
    }

    fn print_to_tty(&self) {
        // my change
        //    println!("uhm {:#?}", &self);
        match self.reg_exp.find(&self.path) {
            Some(result) => {
                let matched_str = &self.path[result.start()..result.end()];

                let colored_match = Green.bold().paint(matched_str).to_string();
                let path = self.path.replace(matched_str, &colored_match);
                // my change
                println!("see the path {}", path);
            }

            None => {
                println!("{}", self.path);
            }
        }
    }
}

fn truncate_working_dir_path(path: String, working_dir_path: &str) -> String {
     println!("path {}", path);
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

pub struct Walker<'a> {
    args: &'a Args,
}

impl<'a> Walker<'a> {
    pub fn new(args: &Args) -> Walker {
        Walker { args }
    }

    //Todo return a string so that we can use the path
    //Todo write file upload logic
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

        let print_thread = thread::spawn(move || -> io::Result<()> {
            for path in rx.iter() {
                PathPrinter::new(path, &reg_exp).print();
            }
            Ok(())
        });
        println!("tx {:?}", print_thread);

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