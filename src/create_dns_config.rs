/// Create the DNS config file for bind

use std::{ fs::File, io::{BufWriter, Write}, };
use crate::{ ProcessedLine, ParsedInfo };

pub fn write_dns_config(parsed_info: &ParsedInfo) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("./db.ionescu").unwrap();
    let mut out = BufWriter::new(&file);

    parsed_info.dns_prefix.iter()
        .try_for_each(|text| writeln!(out, "{}", text))?;

    let longest = compute_max_name_lenght(&parsed_info.ip_lines);
    for line in &parsed_info.ip_lines {
        if let ProcessedLine::Line { number: _, text: _, mac: _, ip, names } = line {
            write_ip_group(&mut out, ip, &names, longest)?;
        }
    }

    parsed_info.dns_suffix.iter()
        .try_for_each(|text| writeln!(out, "{}", text))?;

    Ok(())
}

fn write_ip_group(out: &mut BufWriter<&File>, ip: &str, names: &[&str], longest: usize ) -> Result<(), Box<dyn std::error::Error>> {
    let mut i = names.iter();
    let a = i.next().unwrap();
    let width = longest+1;
    writeln!(out, "{a:width$} IN A     {ip}")?;
    i.try_for_each(|name| writeln!(out, "{0:1$} IN CNAME {2}", name, longest + 1, a))?;
    Ok(())
}

fn compute_max_name_lenght(lines: &[ProcessedLine]) -> usize {
    lines.iter()
        .map(|x| match x {
            ProcessedLine::Line { number: _, text: _, mac: _, ip: _, names } => 
                names.iter().map(|s| s.len()).max().unwrap(),
            _ => 0
        }).max().unwrap()
}
