use crate::directory;
use ansi_term::Colour::Red;
use atty::Stream;
use clap;
use regex::{Regex, RegexBuilder};
use std::path::Path;
use std::process;

#[derive(Debug)]
pub struct ArgMatchesWrapper<'a> {
    matches: clap::ArgMatches,
    file_name: &'a str,
}

#[derive(Debug)]
pub struct Args {
    pub root_path: String,
    pub reg_exp: Regex,
    pub exclude_directories: bool,
    pub exclude_reg_exp: Option<Regex>,
}

impl Args {
    pub fn parse(arg_name: &str) -> Args {
        let args_matches = ArgMatchesWrapper {
            matches: directory::app(),
            file_name: arg_name,
        };

        println!("arg matches {:#?}", args_matches);

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
        }
    }

    fn parse_regex_from_pattern_of(&self, arg_name: &str, error_message: &str) -> Regex {
        let input_pattern = self.matches.value_of(arg_name).unwrap();
        let formatted_pattern = format!(r#"{}"#, input_pattern).to_string();
        let regex_builder = RegexBuilder::new(&formatted_pattern)
            .case_insensitive(false)
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
}
