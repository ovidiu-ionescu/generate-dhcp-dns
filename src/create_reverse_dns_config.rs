/// Create the reverse DNS config file for bind
use std::{
  fs::File,
  io::{BufWriter, Write},
};

use crate::parser::{Line, ParsedInfo};

pub fn write_reverse_dns_config(
  parsed_info: &ParsedInfo, output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  std::fs::create_dir_all(output_dir)?;
  let mut path = std::path::PathBuf::from(output_dir);
  path.push(parsed_info.reverse_dns_file_name);

  let file = File::create(&path)?;
  let mut out = BufWriter::new(&file);

  // write the prefix
  writeln!(out, "{}", parsed_info.dns_prefix)?;

  for line in &parsed_info.ip_lines {
    let Line { ip, names, .. } = line;
    let name = names[0];
    let addr = ip.rsplit('.').next().unwrap();
    if name == "@" {
      writeln!(out, "{addr:3} IN PTR {}.", parsed_info.domain)?;
    } else {
      writeln!(out, "{addr:3} IN PTR {name}.{}.", parsed_info.domain)?;
    }
  }
  println!("✓ Reverse DNS config written to 「{}」", path.display());
  Ok(())
}
