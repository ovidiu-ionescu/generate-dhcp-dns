#![feature(format_args_capture)]
#![feature(stmt_expr_attributes)]

mod create_dhcp_config;
mod create_dns_config;
mod create_reverse_dns_config;
mod parser;
use parser::{parse, ParsedInfo, ProcessedLine};
mod validation;
use validation::validate;

pub fn principal(content: &str) -> Result<(), Box<dyn std::error::Error + '_>> {
    let parsed_info = parse(content)?;
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
