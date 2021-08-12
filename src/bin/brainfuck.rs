use brainfuck::parser::Parser;
use brainfuck::virtual_machine::VM;

use clap::{App, Arg};
use colored::*;

use std::fs;
use std::io;
use std::process;

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

    let file_name = matches.value_of("INPUT").exit_no_file();
    let contents = fs::read_to_string(file_name).exit_bad_file();

    let mut parser = Parser::default();

    parser.parse(&contents).exit_bad_program();

    for warning in parser.warnings() {
        println!(
            "{} {}",
            "warning:".yellow().bold(),
            warning.to_string().bold()
        );
        println!(
            " {} {}:{}",
            "-->".blue().bold(),
            file_name,
            warning.beginning()
        );
        println!("  {}", "|".blue().bold(),);
        println!(
            "{} {} {}",
            warning.beginning().to_string().blue().bold(), // line number
            "|".blue().bold(),
            warning.line()
        );
        println!("  {}", "|".blue().bold(),);
    }

    //VM::new(parser.ir_program()).run(&mut io::stdout(), &mut io::stdin());

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
