use std::collections::HashMap;

use crate::parser::{Line, ParsedInfo};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
  #[error("✗ Mac address {2} on line {0} is duplicate of line {1}")]
  DuplicateMacAddress(usize, usize, String),
  #[error("✗ Ip address {2} on line {0} is duplicate of line {1}")]
  DuplicateIpAddress(usize, usize, String),
  #[error("✗ Host name {2} on line {0} is duplicate of line {1}")]
  DuplicateHostName(usize, usize, String),
  #[error("✗ No domain specified in the configuration (domain)")]
  MissingDomain(),
  #[error("✗ No DNS file specified in the configuration (dns_file_name)")]
  MissingDNSFileName(),
  #[error("✗ No reverse DNS file specified in the configuration (reverse_dns_file_name)")]
  MissingReverseDNSFileName(),
  #[error("✗ No DHCP file specified in the configuration (dhcp_file_name)")]
  MissingDHCPFileName(),
}

pub fn validate(parsed_info: &ParsedInfo) -> Result<(), ValidationError> {
  check_unique_mac(parsed_info)?;
  check_unique_ip(parsed_info)?;
  check_unique_host(parsed_info)?;
  check_required_fields(parsed_info)?;
  Ok(())
}

fn check_unique_mac(parsed_info: &ParsedInfo) -> Result<(), ValidationError> {
  let mut uniq = HashMap::<String, &Line>::new();

  // Check for duplicate MAC addresses
  for ip_line in parsed_info.ip_lines.iter() {
    if let Some(mac) = ip_line.mac {
      // insert returns the old value if the key was already present
      if let Some(Line {
        number: e_number, ..
      }) = uniq.insert(mac.to_lowercase(), ip_line)
      {
        return Err(ValidationError::DuplicateMacAddress(
          ip_line.number,
          *e_number,
          mac.to_string(),
        ));
      }
    }
  }
  Ok(())
}

fn check_unique_ip(parsed_info: &ParsedInfo) -> Result<(), ValidationError> {
  let mut uniq = HashMap::<&str, &Line>::new();

  for ip_line in parsed_info.ip_lines.iter() {
    if let Some(Line {
      number: e_number, ..
    }) = uniq.insert(ip_line.ip, ip_line)
    {
      return Err(ValidationError::DuplicateIpAddress(
        ip_line.number,
        *e_number,
        ip_line.ip.to_string(),
      ));
    }
  }
  Ok(())
}

fn check_unique_host(parsed_info: &ParsedInfo) -> Result<(), ValidationError> {
  let mut uniq = HashMap::<&str, &Line>::new();

  for ip_line in parsed_info.ip_lines.iter() {
    let Line { number, names, .. } = ip_line;
    for name in names {
      if let Some(Line {
        number: e_number, ..
      }) = uniq.insert(name, ip_line)
      {
        return Err(ValidationError::DuplicateHostName(
          *number,
          *e_number,
          String::from(*name),
        ));
      }
    }
  }
  Ok(())
}

fn check_required_fields(parsed_info: &ParsedInfo) -> Result<(), ValidationError> {
  if parsed_info.domain.is_empty() {
    return Err(ValidationError::MissingDomain());
  }
  if parsed_info.dns_file_name.is_empty() {
    return Err(ValidationError::MissingDNSFileName());
  }
  if parsed_info.reverse_dns_file_name.is_empty() {
    return Err(ValidationError::MissingReverseDNSFileName());
  }
  if parsed_info.dhcp_file_name.is_empty() {
    return Err(ValidationError::MissingDHCPFileName());
  }
  Ok(())
}
