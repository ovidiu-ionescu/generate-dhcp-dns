/// Create the reverse DNS config file for bind
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{ParsedInfo, ProcessedLine};

pub fn write_reverse_dns_config(parsed_info: &ParsedInfo) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("./db.0.0.10").unwrap();
    let mut out = BufWriter::new(&file);

    // write the prefix
    parsed_info.dns_prefix.iter().try_for_each(|text| writeln!(out, "{}", text))?;

    for line in &parsed_info.ip_lines {
        if let ProcessedLine::Line { ip, names, .. } = line {
            let name = names[0];
            let addr = ip.rsplit('.').next().unwrap();
            if name == "@" {
                writeln!(out, "{addr:3} IN PTR {}.", parsed_info.domain)?;
            } else {
                writeln!(out, "{addr:3} IN PTR {name}.{}.", parsed_info.domain)?;
            }
        }
    }
    Ok(())
}
