mod parser;
mod validation;
mod create_dns_config;
mod create_reverse_dns_config;
mod create_dhcp_config;

pub fn process<'a>(input: &'a str, input_file_name: &'a str, output_dir: &'a str) -> Result<(), Box<dyn std::error::Error + 'a>> {
  let parsed_info = parser::parser(input)?;
  println!("✓ Parsed configuration file 「{}」", input_file_name);
  //println!("{:#?}", parsed_info);
  validation::validate(&parsed_info)?;
  println!("✓ Validated configuration file 「{}」", input_file_name);
  create_dns_config::write_dns_config(&parsed_info, output_dir)?;
  create_reverse_dns_config::write_reverse_dns_config(&parsed_info, output_dir)?;
  create_dhcp_config::write_dhcp_config(&parsed_info, output_dir)?;
  Ok(())
}

