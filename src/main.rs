#![feature(format_args_capture)]

use std::fs;

mod create_dns_config;
mod parser;
use parser::{ process, ProcessedLine, ParsedInfo };


// the data string is never discarded, leak it and make it static
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn main() {
    let content = string_to_static_str(fs::read_to_string("netconfig.ncf").unwrap());
    let res = process(content);
    match res {
        Ok(parsed_info) => create_output_files(&parsed_info),
        Err(e) => println!("Error parsing file {}", e)
    }
}

fn create_output_files(parsed_info: &ParsedInfo) {
    create_dns_config::write_dns_config(parsed_info);
}

