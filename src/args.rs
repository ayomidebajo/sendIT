use crate::directory;
use ansi_term::Colour::Red;
use atty::Stream;
use clap;
use regex::{Regex, RegexBuilder};
use std::path::Path;
use std::process;


// write tests
#[derive(Debug)]
pub struct ArgMatchesWrapper<'a> {
    matches: clap::ArgMatches,
    file_name: &'a str,
}

#[derive(Debug, Clone)]
pub struct Args {
    pub root_path: String,
    pub reg_exp: Regex,
    pub exclude_directories: bool,
    pub exclude_reg_exp: Option<Regex>,
    pub filename: String,
    pub port_address: String,
}

impl Args {
    pub fn parse() -> Args {
        let app = directory::app();
        let args_matches = ArgMatchesWrapper {
            matches: app.clone(),
            file_name: &app.value_of("PATTERN").unwrap(),
        };
        // println!("arg matches {:#?}", args_matches.matches);

        args_matches.to_args()
    }
}

impl<'a> ArgMatchesWrapper<'a> {
    pub fn to_args(&self) -> Args {
        Args {
            root_path: self.root_path(),
            reg_exp: self.pattern(),
            exclude_directories: self.exclude_direc(),
            exclude_reg_exp: self.exclude_reg_exp_pattern(),
            filename: self.file_name.to_string(),
            port_address: self.port_addr(),
        }
    }

    fn parse_regex_from_pattern_of(&self, arg_name: &str, error_message: &str) -> Regex {
        let input_pattern = self.matches.value_of(arg_name).unwrap();
        let formatted_pattern = format!(r#"{}"#, input_pattern).to_string();
        let regex_builder = RegexBuilder::new(&formatted_pattern)
            .case_insensitive(true)
            .build();

        match regex_builder {
            Ok(reg_exp) => reg_exp,

            Err(_) => {
                let erroneous_pattern = if atty::is(Stream::Stderr) {
                    Red.paint(formatted_pattern).to_string()
                } else {
                    formatted_pattern
                };

                eprintln!("{} {}", error_message, erroneous_pattern);

                process::exit(1);
            }
        }
    }

    pub fn root_path(&self) -> String {
        let root_path = self.matches.value_of("ROOT_PATH").unwrap();

        if Path::new(root_path).is_dir() {
            root_path.to_string()
        } else {
            let erroneous_path = if atty::is(Stream::Stderr) {
                root_path.to_string()
            } else {
                String::from(root_path)
            };

            eprintln!(
                "The specified ROOT_PATH {} is either not accessible or is not a directory",
                erroneous_path
            );

            process::exit(1)
        }
    }
    pub fn pattern(&self) -> Regex {
        self.parse_regex_from_pattern_of("PATTERN", "error, failed to parse")
    }

    pub fn exclude_direc(&self) -> bool {
        self.matches.is_present("exclude-directories")
    }
    pub fn exclude_reg_exp_pattern(&self) -> Option<Regex> {
        if self.matches.is_present("exclude") {
            let reg_exp = self.parse_regex_from_pattern_of(
                "exclude",
                "Failed to parse the pattern provided to the '--exclude (-x)' option:",
            );

            Some(reg_exp)
        } else {
            None
        }
    }
    pub fn port_addr(&self) -> String {
        println!("port {}", self.matches.value_of("PORT").unwrap());
        self.matches.value_of("PORT").unwrap().to_string()
    }
}
