use std::collections::HashMap;
use crate::{ ProcessedLine, ParsedInfo, ParsingError };

pub fn validate<'a, 'b>(parsed_info: &'a ParsedInfo) -> Result<(), ParsingError<'b>> {
    check_unique_mac(parsed_info)?;
    check_unique_ip(parsed_info)?;
    check_unique_host(parsed_info)?;
    Ok(())
}

fn check_unique_mac<'a, 'b>(parsed_info: &'a ParsedInfo) -> Result<(), ParsingError<'b>> {
    let mut uniq = HashMap::<&str, &ProcessedLine>::new();
    
    for ip_line in parsed_info.ip_lines.iter() {
        if let ProcessedLine::Line {number, mac: Some(omac), ..} = ip_line {
            if let Some(ProcessedLine::Line {number: e_number, ..}) = uniq.insert(omac, ip_line) {
                return Err(ParsingError::DuplicateMacAddress(*number, *e_number, String::from(*omac)));
            }
        }
    }
    Ok(())
}

fn check_unique_ip<'a, 'b>(parsed_info: &'a ParsedInfo) -> Result<(), ParsingError<'b>> {
    let mut uniq = HashMap::<&str, &ProcessedLine>::new();
    
    for ip_line in parsed_info.ip_lines.iter() {
        if let ProcessedLine::Line {number, ip, ..} = ip_line {
            if let Some(ProcessedLine::Line {number: e_number, ..}) = uniq.insert(ip, ip_line) {
                return Err(ParsingError::DuplicateIpAddress(*number, *e_number, String::from(*ip)));
            }
        }
    }
    Ok(())
}


fn check_unique_host<'a, 'b>(parsed_info: &'a ParsedInfo) -> Result<(), ParsingError<'b>> {
    let mut uniq = HashMap::<&str, &ProcessedLine>::new();
    
    for ip_line in parsed_info.ip_lines.iter() {
        if let ProcessedLine::Line {number, names, ..} = ip_line {
            for name in names {
                if let Some(ProcessedLine::Line {number: e_number, ..}) = uniq.insert(name, ip_line) {
                    return Err(ParsingError::DuplicateHostName(*number, *e_number, String::from(*name)));
                }
            }
        }
    }
    Ok(())
}
