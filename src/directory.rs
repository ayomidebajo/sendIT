use clap::{Arg, ArgMatches, Command};
use lazy_static::lazy_static;
use std::env;

pub fn app() -> ArgMatches {
    const ABOUT: &str = "
Sendit is a software that allows two different desktops share files between each other";
    lazy_static! {
        static ref WORKING_DIR_PATH: String = working_dir_path();
    }

    Command::new("sendit")
        .version("1.0")
        .author("Ayomide Bajo")
        .about(ABOUT)
        .max_term_width(100)
        .arg(
            Arg::new("PATTERN")
                .help("Find files whose name (path) matches this substring or the regular expression.")
                .index(1)
                .required(true),
        )
       .arg(
            Arg::new("PORT")
            .help("port address of the receiver")
            .index(2)
            .required(true)
       )
         .arg(
            Arg::new("ROOT_PATH")
                .help("Path to the directory to search files inside.")
                .index(3)
                .default_value(&WORKING_DIR_PATH)
                .required(false),
        )
        .arg(
            Arg::new("exclude-directories")
                .help("Exclude paths from the search result which are directories and not files.")
                .short('D')
                .long("exclude-dir-paths"),
        ).arg(
            Arg::new("exclude")
                .help("Exclude files and directories matching this regular expression from the search results.")
                .short('x')
                .takes_value(true)
                .long("exclude")
        )
        .get_matches()
}

pub fn working_dir_path() -> String {
    match env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(_) => String::from("."),
    }
}