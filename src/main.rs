#![feature(format_args_capture)]
use std::fs;

use ::generate_dhcp_dns::principal;

// the data string is never discarded, leak it and make it static
fn string_to_static_str(s: String) -> &'static str { Box::leak(s.into_boxed_str()) }

fn main() {
    let file_name = "netconfig.ncf";
    let content = string_to_static_str(fs::read_to_string(file_name).unwrap());
    if let Err(e) = principal(content) {
        eprintln!("Error parsing file {}: {}", file_name, e);
    }
}
