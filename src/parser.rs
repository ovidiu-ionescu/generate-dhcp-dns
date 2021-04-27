use thiserror::Error;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub enum ProcessedLine<'a> {
    NoOp {
        number: usize,
        text: &'a str,
    },
    Line {
        number: usize,
        text: &'a str,
        mac: Option<&'a str>,
        ip: &'a str,
        names: Vec<&'a str>,
    }
}

pub struct ParsedInfo<'a> {
    pub ip_lines: Vec<ProcessedLine<'a>>,
    pub domain: &'a str,
    pub dns_prefix: Vec<&'a str>,
    pub dns_suffix: Vec<&'a str>,
}

#[derive(Error, Debug)]
pub enum ParsingError<'a> {
    #[error("line {0} does not start with a mac or an ip\n {1}")]
    NoMacOrIp(usize, &'a str),
    #[error("line {0} does not have an IP address\n {1}")]
    NoIpAddress(usize, &'a str),
    #[error("Can not IO")]
    IoError(#[from] std::io::Error),
    #[error("No server names found on line {0}\n {1}")]
    NoServerNames(usize, &'a str),
    #[error("DNS prefix section not terminated")]
    DNSPrefixNotTerminated,
    #[error("DNS suffix section not terminated")]
    DNSSuffixNotTerminated,
}

enum ParsingStatus {
    IpLines,
    DNS_PREFIX,
    DNS_SUFFIX,
}

pub fn process(content: &str) -> Result<ParsedInfo, ParsingError> {

    /*
    let processed_lines: Vec<ProcessedLine> = content.lines()
        .enumerate().map(|(number, text)| Ok(process_line(number + 1, text)?))
        .collect();
    */

    let mut parsing_status = ParsingStatus::IpLines;
    let mut dns_prefix: Vec<&str> = Vec::new();
    let mut dns_suffix: Vec<&str> = Vec::new();

    let mut ip_lines = Vec::new();
    for (number, text) in content.lines().enumerate() {
        match parsing_status {
            ParsingStatus::DNS_PREFIX => {
                if text.starts_with("DNS_PREFIX_END") {
                    parsing_status = ParsingStatus::IpLines;
                } else {
                    dns_prefix.push(text);
                }
            },
            ParsingStatus::DNS_SUFFIX => {
                if text.starts_with("DNS_SUFFIX_END") {
                    parsing_status = ParsingStatus::IpLines;
                } else {
                    dns_suffix.push(text);
                }
            },
            ParsingStatus::IpLines => {
                if text.starts_with("domain") {
                } else
                if text.starts_with("DNS_PREFIX_START") {
                    parsing_status = ParsingStatus::DNS_PREFIX;
                } else
                if text.starts_with("DNS_SUFFIX_START") {
                    parsing_status = ParsingStatus::DNS_SUFFIX;
                } else {
                    ip_lines.push(process_line(number + 1, text)?);
                }
            },
        }
    }

    match parsing_status {
        ParsingStatus::DNS_PREFIX => Err(ParsingError::DNSSuffixNotTerminated),
        ParsingStatus::DNS_SUFFIX => Err(ParsingError::DNSSuffixNotTerminated),
        ParsingStatus::IpLines => Ok(ParsedInfo { ip_lines, domain: "ionescu.net", dns_prefix, dns_suffix })
    }
    
}

fn remove_comment(line: &str) -> &str {
    line.split(';').next().unwrap_or("").trim()
}

fn process_line(number: usize, text: &str) -> Result<ProcessedLine, ParsingError> {
    let active_text = remove_comment(text).trim();
    // ignore lines that have no content other than comments
    if active_text.is_empty() {
        return Ok(ProcessedLine::NoOp { number, text});
    }
    let mut eit = active_text
        .split(' ')
        .filter(|x| !x.is_empty());
    let mut term = eit.next();

    // can this happen? We already checked for an empty line
    if term.is_none() {
        // text is empty, return a noop
        return Ok(ProcessedLine::NoOp{ number, text });
    }

    let mut mac: Option<&str> = None;
    lazy_static! {
        static ref MAC: Regex = Regex::new("^([0-9a-f]{2}:){5}[0-9a-f]{2}$").unwrap();
        static ref IP: Regex = Regex::new(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
    }

    // starting mac address is optional
    if MAC.is_match(term.unwrap()) {
        mac = term;
        term = eit.next();
    }

    // IP address is mandatory
    if term.is_none() || !IP.is_match(term.unwrap()) {
        if mac.is_none() {
            return Err(ParsingError::NoMacOrIp(number, text));
        } else {
            return Err(ParsingError::NoIpAddress(number, text));
        }
    }
    let ip = term.unwrap();

    let names: Vec<&str> = eit.collect();
    if names.is_empty() {
        return Err(ParsingError::NoServerNames(number, text));
    }
    Ok(ProcessedLine::Line { number, text, mac, ip, names })
}

macro_rules! assert_err {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => (),
            ref e => panic!("expected `{}` but got `{:?}`", stringify!($($pattern)+), e),
        }
    }
}

#[test]
fn test_remove_comment() {
    assert_eq!(remove_comment("text ; comment"), "text");
    assert_eq!(remove_comment("text"), "text");
    assert_eq!(remove_comment(""), "");
    assert_eq!(remove_comment("; comment"), "");
}

#[test]
fn test_process_line() {
    println!("{:?}", process_line(2, "  00:00:af:de:12:34 127.0.0.1 server.com"));
    println!("{:?}", process_line(0, "; Network configuration"));

    assert_err!(process_line(2, " server.com ; comment"), Err(ParsingError::NoMacOrIp(2, _)));
    assert_err!(process_line(2, " 00:00:00:00:00:00 server.com ; comment"), Err(ParsingError::NoMacOrIp(2, _)));
}
