mod scanner;
mod parser;
mod ream;
mod csv;

use std::{env, fs};
use clap::{Arg, App};
use csv::*;
use crate::scanner::*;
use crate::parser::*;

fn main() {

    // let result = ream2csv("# test");
    // println!("{:?}", result);

    let file = fs::read_to_string("./example/test.md").unwrap();
    let mut parser = Parser::new(&file);
    let result = parser.parse_entry().unwrap().ok_or_else(|| panic!("something went wrong")).unwrap();




    println!("{:?}", result.flatten());


    // let matches = App::new("REAM Core")
    //     .version("0.1.0")
    //     .author("Chih-Ming Louis Lee <louis@chihminglee.com>")
    //     .about("Ream encoder and decoder")
    //     .arg(
    //         Arg::new("input")
    //             .long("input")
    //             .short('i')
    //             .value_name("FILE")
    //             .takes_value(true)
    //     )
    //     .arg(
    //         Arg::new("format")
    //             .long("format")
    //             .short('f')
    //             .takes_value(true)
    //             .possible_values(&[
    //                 "Json",
    //             ])
    //     )
    //     .get_matches();

    // let path = match matches.value_of("input") {
    //     Some(p) => p,
    //     _ => "./example/test.md",
    // };

    // let file = fs::read_to_string(path).unwrap();
    // let mut parser = Parser::new(&file);

    // match matches.value_of("format") {
    //     _ => {
    //         let result = parser.parse_entry().unwrap().ok_or_else(|| panic!("something went wrong"));
    //         let se = serde_json::to_string_pretty(&result).unwrap();
    //         println!("{}", &se);
    //     },
    // };
}
