use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

#[derive(Debug)]
pub enum ProcessedLine<'a> {
    NoOp { number: usize, text: &'a str },
    Line { number: usize, text: &'a str, mac: Option<&'a str>, ip: &'a str, names: Vec<&'a str> },
}

pub struct ParsedInfo<'a> {
    pub ip_lines:              Vec<ProcessedLine<'a>>,
    pub domain:                &'a str,
    pub dns_prefix:            Vec<&'a str>,
    pub dns_suffix:            Vec<&'a str>,
    pub dhcp_prefix:           Vec<&'a str>,
    pub dns_file_name:         &'a str,
    pub reverse_dns_file_name: &'a str,
    pub dhcp_file_name:        &'a str,
}

#[derive(Error, Debug, PartialEq)]
pub enum ParsingError<'a> {
    #[error("line {0} does not start with a mac or an ip\n {1}")]
    NoMacOrIp(usize, &'a str),
    #[error("line {0} does not have an IP address\n {1}")]
    NoIpAddress(usize, &'a str),
    #[error("No server names found on line {0}\n {1}")]
    NoServerNames(usize, &'a str),
    #[error("DNS prefix section not terminated")]
    DNSPrefixNotTerminated,
    #[error("DNS suffix section not terminated")]
    DNSSuffixNotTerminated,
    #[error("DHCP prefix section not terminated")]
    DHCPPrefixNotTerminated,
    #[error("No parent domain specified, add line 'domain foo.net'")]
    NoParentDomain,
    #[error("No DNS file name specified, add line 'dns_file_name db.foo'")]
    NoDNSFileName,
    #[error("No reverse DNS file name specified, add line 'reverse_dns_file_name db.0.0.10'")]
    NoReverseDNSFileName,
    #[error("No DHCP file name specified, add line 'dhcp_file_name reservations.conf'")]
    NoDHCPFileName,
    #[error("line {0} bad {2} specifier, more than one name\n {1}")]
    BadValueSpecifier(usize, &'a str, &'a str),
}

/// Represents the current parsing stage. IpLines are the regular lines
/// like:
/// 10:00:00:00:00:aa 10.0.0.1 host1.net host2.net ; Comment
/// or:
/// domain mydomain.net
/// The others represent all the other sections
enum ParsingStatus {
    IpLines,
    DnsPrefix,
    DnsSuffix,
    DhcpPrefix,
}

pub fn parse(content: &str) -> Result<ParsedInfo, ParsingError> {
    let mut parsing_status = ParsingStatus::IpLines;
    let mut dns_prefix: Vec<&str> = Vec::new();
    let mut dns_suffix: Vec<&str> = Vec::new();
    let mut dhcp_prefix: Vec<&str> = Vec::new();
    let mut domain: Option<&str> = None;
    let mut dns_file_name = None::<&str>;
    let mut reverse_dns_file_name = None::<&str>;
    let mut dhcp_file_name = None::<&str>;

    let mut ip_lines = Vec::new();
    for (number, text) in content.lines().enumerate() {
        let token = get_token(text);
        #[rustfmt::skip]
        match (&parsing_status, token) {
            (ParsingStatus::DnsPrefix, "DNS_PREFIX_END")   => parsing_status = ParsingStatus::IpLines,
            (ParsingStatus::DnsPrefix, _)                  => dns_prefix.push(text),

            (ParsingStatus::DnsSuffix, "DNS_SUFFIX_END")   => parsing_status = ParsingStatus::IpLines,
            (ParsingStatus::DnsSuffix, _)                  => dns_suffix.push(text),

            (ParsingStatus::DhcpPrefix, "DHCP_PREFIX_END") => parsing_status = ParsingStatus::IpLines,
            (ParsingStatus::DhcpPrefix, _)                 => dhcp_prefix.push(text),

            (ParsingStatus::IpLines, _)                    => {
                match token {
                    "DNS_PREFIX_START"      => parsing_status = ParsingStatus::DnsPrefix,
                    "DNS_SUFFIX_START"      => parsing_status = ParsingStatus::DnsSuffix,
                    "DHCP_PREFIX_START"     => parsing_status = ParsingStatus::DhcpPrefix,

                    "domain"                => domain = get_value(text, number, "parent domain")?,
                    "dns_file_name"         => dns_file_name = get_value(text, number, "DNS file name")?,
                    "reverse_dns_file_name" => reverse_dns_file_name = get_value(text, number, "reverse DNS file name")?,
                    "dhcp_file_name"        => dhcp_file_name = get_value(text, number, "DHCP file name")?,

                    _                       => ip_lines.push(process_line(number + 1, text)?),
                }
            },
        }
    }

    if domain.is_none() {
        return Err(ParsingError::NoParentDomain);
    }
    if dns_file_name.is_none() {
        return Err(ParsingError::NoDNSFileName);
    }
    if reverse_dns_file_name.is_none() {
        return Err(ParsingError::NoReverseDNSFileName);
    }
    if dhcp_file_name.is_none() {
        return Err(ParsingError::NoDHCPFileName);
    }

    match parsing_status {
        ParsingStatus::DnsPrefix => Err(ParsingError::DNSPrefixNotTerminated),
        ParsingStatus::DnsSuffix => Err(ParsingError::DNSSuffixNotTerminated),
        ParsingStatus::DhcpPrefix => Err(ParsingError::DHCPPrefixNotTerminated),
        ParsingStatus::IpLines => Ok(ParsedInfo {
            ip_lines,
            domain: domain.unwrap(),
            dns_prefix,
            dns_suffix,
            dhcp_prefix,
            dns_file_name: dns_file_name.unwrap(),
            reverse_dns_file_name: reverse_dns_file_name.unwrap(),
            dhcp_file_name: dhcp_file_name.unwrap(),
        }),
    }
}

fn remove_comment(line: &str) -> &str { line.split(';').next().unwrap_or("").trim() }

fn get_value<'a>(text: &'a str, number: usize, value_name: &'static str) -> Result<Option<&'a str>, ParsingError<'a>> {
    let active_text = remove_comment(text).trim();
    let mut i = active_text.split_whitespace().filter(|x| !x.is_empty());

    // skip past the key, it's already been handled by the caller
    i.next();
    let value = i.next();
    if value.is_none() {
        return Err(ParsingError::BadValueSpecifier(number + 1, text, value_name));
    }
    if i.next().is_some() {
        // there's unexpected trailing text
        return Err(ParsingError::BadValueSpecifier(number + 1, text, value_name));
    }
    Ok(value)
}

fn get_token(text: &str) -> &str { text.split_whitespace().next().unwrap_or("") }

fn process_line(number: usize, text: &str) -> Result<ProcessedLine, ParsingError> {
    let active_text = remove_comment(text).trim();
    // ignore lines that have no content other than comments
    if active_text.is_empty() {
        return Ok(ProcessedLine::NoOp { number, text });
    }
    let mut eit = active_text.split(' ').filter(|x| !x.is_empty());
    let mut term = eit.next();

    // can this happen? We already checked for an empty line
    if term.is_none() {
        // text is empty, return a noop
        return Ok(ProcessedLine::NoOp { number, text });
    }

    let mut mac: Option<&str> = None;
    lazy_static! {
        static ref MAC: Regex = Regex::new("^([0-9a-f]{2}:){5}[0-9a-f]{2}$").unwrap();
        static ref IP: Regex =
            Regex::new(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
    }

    // starting mac address is optional
    if MAC.is_match(term.unwrap()) {
        mac = term;
        term = eit.next();
    }

    // IP address is mandatory
    if term.is_none() || !IP.is_match(term.unwrap()) {
        return match mac {
            None => Err(ParsingError::NoMacOrIp(number, text)),
            _ => Err(ParsingError::NoIpAddress(number, text)),
        };
    }
    let ip = term.unwrap();

    let names: Vec<&str> = eit.collect();
    if names.is_empty() {
        return Err(ParsingError::NoServerNames(number, text));
    }
    Ok(ProcessedLine::Line { number, text, mac, ip, names })
}

#[cfg(test)]
mod tests {

    use super::{process_line, remove_comment, ParsingError};

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
        assert_err!(process_line(2, " 00:00:00:00:00:00 server.com ; comment"), Err(ParsingError::NoIpAddress(2, _)));
    }
}
