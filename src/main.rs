mod error;
mod parser;
mod ream;
mod scanner;

use crate::parser::*;
use crate::scanner::*;
use clap::{App, Arg};
use std::fs::File;
use std::io::Write;
use std::{env, fs};

fn main() {
    let matches = App::new("REAM Core")
        .version("0.4.0")
        .author("Chih-Ming Louis Lee <louis@chihminglee.com>")
        .about("Ream encoder and decoder")
        .arg(
            Arg::new("input")
                .long("input")
                .short('i')
                .value_name("FILE")
                .takes_value(true),
        )
        .arg(
            Arg::new("format")
                .long("format")
                .short('f')
                .takes_value(true)
                .possible_values(&["AST", "CSV"]),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .takes_value(true),
        )
        .arg(
            Arg::new("print")
                .long("print")
                .short('p')
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('d')
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    // debug starts
    let debug = matches.is_present("debug");
    if !debug {
        // debug ends

        let input_path = match matches.value_of("input") {
            Some(p) => p,
            _ => "./example/test.md",
        };

        let output_path = match matches.value_of("output") {
            Some(p) => p,
            _ => panic!("Missing path for output file"),
        };

        let print = matches.is_present("print");

        let file = fs::read_to_string(input_path).unwrap();
        let mut parser = Parser::new(&file);

        let output_text = match matches.value_of("format") {
            Some(f) => {
                let result = parser.parse_entry().unwrap();
                match f {
                    "AST" => result.unwrap().to_ast_str().unwrap(),
                    "CSV" => result.unwrap().to_csv_str().unwrap(),
                    _ => panic!("Output format not supported"),
                }
            }
            _ => panic!("Missing output format"),
        };

        fs::write(output_path, &output_text).expect("unable to write");

        if print {
            println!("{:#?}", output_text);
        }

    // debug starts
    } else {
        debug_fun();
    }
    // debug ends
}

fn debug_fun() {
    let file = fs::read_to_string("./example/test.md").unwrap();
    let mut parser = Parser::new(&file);
    let entry = parser.parse_entry().unwrap().unwrap();
    let result = entry.flatten_entry();

    println!("{:?}", result);
}
