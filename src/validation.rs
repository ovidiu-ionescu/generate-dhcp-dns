use std::collections::HashMap;
use crate::{ ProcessedLine, ParsedInfo, ParsingError };

pub fn validate<'a, 'b>(parsed_info: &'a ParsedInfo) -> Result<(), ParsingError<'b>> {
    check_unique_mac(parsed_info)?;
    Ok(())
}

fn check_unique_mac<'a, 'b>(parsed_info: &'a ParsedInfo) -> Result<(), ParsingError<'b>> {
    let mut uniq = HashMap::<&str, &ProcessedLine>::new();
    
    let i = parsed_info.ip_lines.iter()
        .filter(|l| match l { 
            ProcessedLine::Line {number: _, text: _, mac, ip: _, names: _} => mac.is_some(), 
            _ => false, 
        });

    for ip_line in i {
        if let ProcessedLine::Line {number, text: _, mac: omac, ip: _, names: _} = ip_line {
            let mac = omac.unwrap();
        match uniq.get(mac) {
            Some(line) => {
                if let ProcessedLine::Line {number: e_number, text: _, mac: _, ip: _, names: _} = line {
                    return Err(ParsingError::DuplicateMacAddress(*number, *e_number, String::from(mac)));
                }
            },
            None => { uniq.insert(mac, ip_line); },
        }
        }
    }
    Ok(())
}
