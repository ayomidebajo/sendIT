use clap::{Arg, ArgMatches, Command};
use lazy_static::lazy_static;
use std::env;



pub fn app() -> ArgMatches {
    const ABOUT: &str = "
Sendit is a software that allows two different desktop to share files between each other";
    lazy_static! {
        static ref WORKING_DIR_PATH: String = working_dir_path();
    }

    Command::new("sendit")
        .version("1.0")
        .author("Ayomide Bajo")
        .about(ABOUT)
        .max_term_width(80)
        .arg(
            Arg::new("PATTERN")
                .help("Find files whose name (path) matches this substring or the regular expression.")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("ROOT_PATH")
                .help("Path to the directory to search files inside.")
                .index(2)
                .default_value(&WORKING_DIR_PATH)
                .required(false),
        ).get_matches()
}

pub fn working_dir_path() -> String {
    match env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(_) => String::from("."),
    }
}