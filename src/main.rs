mod less_naive_parser;
mod string_utils;
mod virtual_machine;

use clap::{App, Arg};
use colored::*;
use less_naive_parser::Parts;
use std::fs;
use std::io;
use std::process;
use virtual_machine::VM;

fn main() {
    let matches = App::new("brainfuck")
        .author(clap::crate_authors!())
        .version(clap::crate_version!())
        .about("A brainfuck interpreter written in Rust")
        .arg(
            Arg::with_name("INPUT")
                .help("Program to interpret")
                .index(1),
        )
        .get_matches();

    let contents = fs::read_to_string(matches.value_of("INPUT").exit_no_file()).exit_bad_file();

    //println!("{}", contents.parse::<Parts>().exit_bad_program());
    VM::new(&contents.parse::<Parts>().exit_bad_program()).run(&mut io::stdout(), &mut io::stdin());

    process::exit(exitcode::OK);
}

pub trait OptionError<T> {
    fn exit_no_file(self) -> T;
}

impl<T> OptionError<T> for Option<T> {
    fn exit_no_file(self) -> T {
        match self {
            Some(val) => val,
            None => {
                println!("{} no input file", "fatal error:".red().bold());
                process::exit(exitcode::NOINPUT);
            }
        }
    }
}

trait ResultError<T> {
    fn exit_bad_file(self) -> T;
    fn exit_bad_program(self) -> T;
}

impl<T, E> ResultError<T> for Result<T, E>
where
    E: std::error::Error,
{
    fn exit_bad_file(self) -> T {
        match self {
            Ok(val) => val,
            Err(error) => {
                println!("{} {}", "fatal error:".red().bold(), error);
                process::exit(exitcode::NOINPUT);
            }
        }
    }

    fn exit_bad_program(self) -> T {
        match self {
            Ok(val) => val,
            Err(error) => {
                println!("{} {}", "fatal error:".red().bold(), error);
                process::exit(exitcode::DATAERR);
            }
        }
    }
}
