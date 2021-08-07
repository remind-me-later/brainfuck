mod executable;
mod instructions;
mod lexemes;
mod position;

use clap::{crate_authors, crate_version, App, Arg};
use colored::*;
use executable::Executable;
use instructions::Instructions;
use lexemes::Lexemes;
use std::fs;
use std::io;
use std::process;

fn main() {
    let matches = App::new("brainfuck")
        .author(crate_authors!())
        .version(crate_version!())
        .about("A brainfuck interpreter written in Rust")
        .arg(
            Arg::with_name("INPUT")
                .help("Program to interpret")
                .index(1),
        )
        .arg(
            Arg::with_name("naive")
                .short("n")
                .long("naive")
                .help("Use a naive interpreter, use this if the default fails"),
        )
        .get_matches();

    let contents =
        fs::read_to_string(matches.value_of("INPUT").exit_on_no_file()).exit_on_bad_file();

    if matches.is_present("naive") {
        contents
            .parse::<Instructions>()
            .exit_on_bad_program()
            .execute(&mut io::stdout(), &mut io::stdin());
    } else {
        contents
            .parse::<Lexemes>()
            .exit_on_bad_program()
            .execute(&mut io::stdout(), &mut io::stdin());
    }

    process::exit(exitcode::OK);
}

pub trait OptionError<T> {
    fn exit_on_no_file(self) -> T;
}

impl<T> OptionError<T> for Option<T> {
    fn exit_on_no_file(self) -> T {
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
    fn exit_on_bad_file(self) -> T;
    fn exit_on_bad_program(self) -> T;
}

impl<T, E> ResultError<T> for Result<T, E>
where
    E: std::error::Error,
{
    fn exit_on_bad_file(self) -> T {
        match self {
            Ok(val) => val,
            Err(error) => {
                println!("{} {}", "fatal error:".red().bold(), error);
                process::exit(exitcode::NOINPUT);
            }
        }
    }

    fn exit_on_bad_program(self) -> T {
        match self {
            Ok(val) => val,
            Err(error) => {
                println!("{} {}", "fatal error:".red().bold(), error);
                process::exit(exitcode::DATAERR);
            }
        }
    }
}
