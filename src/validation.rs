use std::collections::HashMap;

use thiserror::Error;

use crate::{ParsedInfo, ProcessedLine};

#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("Mac address {2} on line {0} is duplicate of line {1}")]
    DuplicateMacAddress(usize, usize, String),
    #[error("Ip address {2} on line {0} is duplicate of line {1}")]
    DuplicateIpAddress(usize, usize, String),
    #[error("Host name {2} on line {0} is duplicate of line {1}")]
    DuplicateHostName(usize, usize, String),
}

pub fn validate(parsed_info: &ParsedInfo) -> Result<(), ValidationError> {
    check_unique_mac(parsed_info)?;
    check_unique_ip(parsed_info)?;
    check_unique_host(parsed_info)?;
    Ok(())
}

fn check_unique_mac(parsed_info: &ParsedInfo) -> Result<(), ValidationError> {
    let mut uniq = HashMap::<&str, &ProcessedLine>::new();

    for ip_line in parsed_info.ip_lines.iter() {
        if let ProcessedLine::Line { number, mac: Some(omac), .. } = ip_line {
            if let Some(ProcessedLine::Line { number: e_number, .. }) = uniq.insert(omac, ip_line) {
                return Err(ValidationError::DuplicateMacAddress(*number, *e_number, String::from(*omac)));
            }
        }
    }
    Ok(())
}

fn check_unique_ip(parsed_info: &ParsedInfo) -> Result<(), ValidationError> {
    let mut uniq = HashMap::<&str, &ProcessedLine>::new();

    for ip_line in parsed_info.ip_lines.iter() {
        if let ProcessedLine::Line { number, ip, .. } = ip_line {
            if let Some(ProcessedLine::Line { number: e_number, .. }) = uniq.insert(ip, ip_line) {
                return Err(ValidationError::DuplicateIpAddress(*number, *e_number, String::from(*ip)));
            }
        }
    }
    Ok(())
}

fn check_unique_host(parsed_info: &ParsedInfo) -> Result<(), ValidationError> {
    let mut uniq = HashMap::<&str, &ProcessedLine>::new();

    for ip_line in parsed_info.ip_lines.iter() {
        if let ProcessedLine::Line { number, names, .. } = ip_line {
            for name in names {
                if let Some(ProcessedLine::Line { number: e_number, .. }) = uniq.insert(name, ip_line) {
                    return Err(ValidationError::DuplicateHostName(*number, *e_number, String::from(*name)));
                }
            }
        }
    }
    Ok(())
}
