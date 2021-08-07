mod executable;
mod instructions;

use clap::{crate_authors, crate_version, App, Arg};
use colored::*;
use executable::Executable;
use instructions::Instructions;
use std::fs;
use std::process;

fn main() {
    let matches = App::new("brainfuck")
        .author(crate_authors!())
        .version(crate_version!())
        .about("A brainfuck interpreter written in Rust")
        .arg(
            Arg::with_name("INPUT")
                .help("Program to interpret")
                .required(true)
                .index(1),
        )
        .get_matches();

    let contents =
        fs::read_to_string(matches.value_of("INPUT").unwrap()).exit_error(exitcode::NOINPUT);

    contents
        .parse::<Instructions>()
        .exit_error(exitcode::DATAERR)
        .execute(&mut std::io::stdout(), &mut std::io::stdin());

    process::exit(exitcode::OK);
}

trait ExitError<T, E> {
    fn exit_error(self, exit_code: exitcode::ExitCode) -> T;
}

impl<T, E> ExitError<T, E> for std::result::Result<T, E>
where
    E: std::error::Error,
{
    fn exit_error(self, exit_code: exitcode::ExitCode) -> T {
        match self {
            Ok(val) => return val,
            Err(error) => {
                println!("{} {}", "error:".red().bold(), error);
                process::exit(exit_code);
            }
        }
    }
}
