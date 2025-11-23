/// Create the DHCP config file for dhcpd
use std::{
  fs::File,
  io::{BufWriter, Write},
};

use crate::parser::{Line, ParsedInfo};

pub fn write_dhcp_config(
  parsed_info: &ParsedInfo, output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  std::fs::create_dir_all(output_dir)?;
  let mut path = std::path::PathBuf::from(output_dir);
  path.push(parsed_info.dhcp_file_name);

  let file = File::create(&path)?;
  let mut out = BufWriter::new(&file);

  writeln!(out, "{}", parsed_info.dhcp_prefix)?;

  for line in &parsed_info.ip_lines {
    let Line { mac, names, .. } = line;
    let name = names[0];
    if let Some(mac_address) = mac {
      writeln!(out)?;
      if name == "@" {
        writeln!(out, "host {} {{", parsed_info.domain)?;
        writeln!(out, "  hardware ethernet {};", mac_address)?;
        writeln!(out, "  fixed-address {};", parsed_info.domain)?;
      } else {
        writeln!(out, "host {name} {{")?;
        writeln!(out, "  hardware ethernet {};", mac_address)?;
        writeln!(out, "  fixed-address {name}.{};", parsed_info.domain)?;
      }
      writeln!(out, "}}")?;
    }
  }
  println!("✓ DHCP config written to 「{}」", path.display());
  Ok(())
}
