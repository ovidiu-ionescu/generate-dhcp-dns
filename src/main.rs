#![feature(format_args_capture)]

use std::fs;

mod create_dns_config;
mod parser;
use parser::{ process, ProcessedLine };


// the data string is never discarded, leak it and make it static
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn main() {
    let content = string_to_static_str(fs::read_to_string("netconfig.ncf").unwrap());
    let res = process(content);
    match res {
        Ok(lines) => create_output_files(&lines),
        Err(e) => println!("Error parsing file {}", e)
    }
}

fn create_output_files(lines: &[ProcessedLine]) {
    create_dns_config::write_dns_config(lines);
}

