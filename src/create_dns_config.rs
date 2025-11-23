/// Create the DNS config file for bind
use std::{
  fs::File,
  io::{BufWriter, Write},
};

use crate::parser::{Line, ParsedInfo};

pub fn write_dns_config(
  parsed_info: &ParsedInfo, output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  std::fs::create_dir_all(output_dir)?;
  let mut path = std::path::PathBuf::from(output_dir);
  path.push(parsed_info.dns_file_name);

  let file = File::create(&path)?;
  let mut out = BufWriter::new(&file);

  writeln!(out, "{}", parsed_info.dns_prefix)?;

  let longest = compute_max_name_lenght(&parsed_info.ip_lines);
  for line in &parsed_info.ip_lines {
    let Line { ip, names, .. } = line;
    write_ip_group(&mut out, ip, names, longest)?;
  }

  if !parsed_info.dns_suffix.is_empty() {
    writeln!(out)?;
  }

  writeln!(out, "{}", parsed_info.dns_suffix)?;

  println!("✓ DNS config written to 「{}」", path.display());
  Ok(())
}

fn write_ip_group(
  out: &mut BufWriter<&File>, ip: &str, names: &[&str], longest: usize,
) -> Result<(), Box<dyn std::error::Error>> {
  let mut i = names.iter();
  let a = i.next().unwrap();
  let width = longest + 1;
  writeln!(out, "{a:width$} IN A     {ip}")?;
  i.try_for_each(|name| writeln!(out, "{0:1$} IN CNAME {2}", name, longest + 1, a))?;
  Ok(())
}

fn compute_max_name_lenght(lines: &[Line]) -> usize {
  lines
    .iter()
    .map(|x| {
      let Line { names, .. } = x;
      names.iter().map(|s| s.len()).max().unwrap()
    })
    .max()
    .unwrap()
}
