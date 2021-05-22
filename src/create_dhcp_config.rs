/// Create the DHCP config file for dhcpd
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{ParsedInfo, ProcessedLine};

pub fn write_reverse_dns_config(parsed_info: &ParsedInfo) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("./oi_reservations.conf").unwrap();
    let mut out = BufWriter::new(&file);

    parsed_info.dhcp_prefix.iter().try_for_each(|text| writeln!(out, "{}", text))?;

    for line in &parsed_info.ip_lines {
        if let ProcessedLine::Line { mac, names, .. } = line {
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
    }
    Ok(())
}
