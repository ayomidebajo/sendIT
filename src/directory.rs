use clap::{ Command, Arg, ArgMatches};
use lazy_static::lazy_static;
use std::env;

const ABOUT: &str = "
Find Files (ff) utility recursively searches the files whose names match the
specified RegExp pattern in the provided directory (defaults to the current
directory if not provided).";

pub fn app() -> ArgMatches {
    lazy_static! {
        static ref WORKING_DIR_PATH: String = working_dir_path();
    }

    Command::new("ff")
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