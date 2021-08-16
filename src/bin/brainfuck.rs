use brainfuck::parser::Parser;
use brainfuck::parser_error::ParserResult;
use brainfuck::virtual_machine::VM;

use clap::{App, Arg};
use colored::*;
use line_col::LineColLookup;

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
    let text = fs::read_to_string(file_name).exit_bad_file(file_name);
    let lookup = LineColLookup::new(&text);

    let mut parser = Parser::default();

    parser.parse(&text).exit_parser(file_name, &text, &lookup);

    for warning in parser.warnings() {
        eprintln!(
            "{} {}",
            "warning:".yellow().bold(),
            warning.to_string().bold()
        );
        print_warning_line(
            file_name,
            &text,
            warning.beginning(),
            warning.end(),
            &lookup,
        );
    }

    //println!("{}", parser.ir());

    VM::new(parser.ir()).run(&mut io::stdout(), &mut io::stdin());

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
                eprintln!("{} {}", "fatal error:".red().bold(), "no input file".bold());
                process::exit(exitcode::NOINPUT);
            }
        }
    }
}

trait ResultError<T: std::fmt::Debug> {
    fn exit_bad_file(self, file_name: &str) -> T;
}

impl<T, E> ResultError<T> for Result<T, E>
where
    T: std::fmt::Debug,
    E: std::error::Error,
{
    fn exit_bad_file(self, file_name: &str) -> T {
        match self {
            Ok(val) => val,
            Err(error) => {
                eprintln!(
                    "{} {}: {}",
                    "fatal error:".red().bold(),
                    error.to_string().bold(),
                    file_name.bold()
                );
                process::exit(exitcode::NOINPUT);
            }
        }
    }
}
trait ExitParserError {
    fn exit_parser(self, file_name: &str, text: &str, lookup: &LineColLookup);
}

impl ExitParserError for ParserResult {
    fn exit_parser(self, file_name: &str, text: &str, lookup: &LineColLookup) {
        match self {
            Ok(_) => (),
            Err(error) => {
                eprintln!(
                    "{} {}",
                    "fatal error:".red().bold(),
                    error.to_string().bold(),
                );

                print_warning_line(
                    file_name,
                    text,
                    error.beginning(),
                    error.beginning(),
                    lookup,
                );

                process::exit(exitcode::DATAERR);
            }
        }
    }
}

fn print_warning_line(
    file_name: &str,
    text: &str,
    beginning: usize,
    end: usize,
    lookup: &LineColLookup,
) {
    let (line_b, column_b) = lookup.get(beginning);
    let (line_e, column_e) = lookup.get(end);
    let line_str = text.lines().nth(line_b - 1).unwrap();
    let line_b_spaces = " ".repeat(line_b.to_string().len());

    eprintln!(
        " {} {}:{}:{}",
        "-->".blue().bold(),
        file_name,
        line_b,
        column_b
    );
    eprintln!(" {} {}", line_b_spaces, "|".blue().bold(),);
    eprintln!(
        " {} {} {}",
        line_b.to_string().blue().bold(), // line number
        "|".blue().bold(),
        line_str
    );
    eprintln!(
        " {} {} {}{}",
        line_b_spaces,
        "|".blue().bold(),
        " ".repeat(column_b - 1),
        "^".repeat(column_e - column_b + 1).yellow().bold()
    );
}
