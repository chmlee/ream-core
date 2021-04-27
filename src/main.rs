mod scanner;
mod parser;
mod ream;

use std::{env, fs};
use std::fs::File;
use std::io::Write;
use clap::{Arg, App};
use crate::scanner::*;
use crate::parser::*;

fn main() {

    let matches = App::new("REAM Core")
        .version("0.1.0")
        .author("Chih-Ming Louis Lee <louis@chihminglee.com>")
        .about("Ream encoder and decoder")
        .arg(
            Arg::new("input")
                .long("input")
                .short('i')
                .value_name("FILE")
                .takes_value(true)
        )
        .arg(
            Arg::new("format")
                .long("format")
                .short('f')
                .takes_value(true)
                .possible_values(&[
                    "JSON",
                    "CSV",
                ])
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .takes_value(true)
        )
        .arg(
            Arg::new("print")
                .long("print")
                .short('p')
                .required(false)
                .takes_value(false)
        )
        .get_matches();

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
            let result = parser.parse_entry().unwrap().ok_or_else(|| panic!("something went wrong"));
            match f {
                "JSON" => {
                    serde_json::to_string(&result).unwrap()
                },
                "CSV" => {
                    result.unwrap().to_csv()
                },
                _ => panic!("Output format not supported"),
            }
        },
        None => panic!("Missing output format")
    };

    fs::write(output_path, &output_text).expect("unable to write");

    if print {
        println!("{:#?}", output_text);
    }
}
