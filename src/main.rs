#![feature(format_args_capture)]

use std::fs;

mod create_dns_config;
mod create_reverse_dns_config;
mod create_dhcp_config;
mod parser;
use parser::{ process, ProcessedLine, ParsedInfo, ParsingError, };
mod validation;
use validation::validate;


// the data string is never discarded, leak it and make it static
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn main() {
    let file_name = "netconfig.ncf";
    if let Err(e) = principal(file_name) {
        eprintln!("Error parsing file {}: {}", file_name, e);
    }
}

fn principal(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = string_to_static_str(fs::read_to_string(file_name).unwrap());
    let parsed_info = process(content)?;
    validate(&parsed_info)?;
    create_output_files(&parsed_info)?;
    Ok(())
}

fn create_output_files(parsed_info: &ParsedInfo) -> Result<(), Box<dyn std::error::Error>> {
    create_dns_config::write_dns_config(parsed_info)?;
    create_reverse_dns_config::write_reverse_dns_config(parsed_info)?;
    create_dhcp_config::write_reverse_dns_config(parsed_info)?;
    Ok(())
}

