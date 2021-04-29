#![feature(format_args_capture)]

use std::fs;

mod create_dns_config;
mod create_reverse_dns_config;
mod create_dhcp_config;
mod parser;
use parser::{ process, ProcessedLine, ParsedInfo };


// the data string is never discarded, leak it and make it static
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = "netconfig.ncf";
    let content = string_to_static_str(fs::read_to_string(file_name).unwrap());
    let res = process(content);
    match res {
        Ok(parsed_info) => create_output_files(&parsed_info)?,
        Err(e) => println!("Error parsing file {}: {}", file_name, e)
    }
    Ok(())
}

fn create_output_files(parsed_info: &ParsedInfo) -> Result<(), Box<dyn std::error::Error>> {
    create_dns_config::write_dns_config(parsed_info)?;
    create_reverse_dns_config::write_reverse_dns_config(parsed_info)?;
    create_dhcp_config::write_reverse_dns_config(parsed_info)?;
    Ok(())
}

