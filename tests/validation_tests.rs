use ::orgncf_generator::process;
use indoc::{formatdoc, indoc};
mod common;

fn proc(input: &str) -> Result<(), Box<dyn std::error::Error + '_>> {
    process(&input, "test_input.ncf", "output_dir")
}

const EPILOGUE: &str = indoc! {r#"
      domain foo.net
      dns_file_name db.foo
      reverse_dns_file_name db.0.0.10
      dhcp_file_name reservations.conf
      DNS_PREFIX """ """
      DNS_SUFFIX """ """
      DHCP_PREFIX """ """
    "#};

#[test]
fn test_duplicate_ip() {
    let s = formatdoc! {r#"
      10:00:00:00:00:aa 10.0.0.1 host1.net ; Comment
      10:00:00:00:00:bb 10.0.0.1 host2.net
      10:00:00:00:00:cc 10.0.0.3 host3.net

      {EPILOGUE}
    "#};
    check_err!(proc(&s), "✗ Ip address 10.0.0.1 on line 2 is duplicate of line 1");
}

#[test]
fn test_duplicate_mac() {
    let s = formatdoc! {r#"
      10:00:00:00:00:aa 10.0.0.1 host1.net ; Comment
      10:00:00:00:00:aa 10.0.0.2 host2.net
      10:00:00:00:00:cc 10.0.0.3 host3.net

      {EPILOGUE}
    "#};
    check_err!(proc(&s), "✗ Mac address 10:00:00:00:00:aa on line 2 is duplicate of line 1");
}

#[test]
fn test_duplicate_hostname() {
    let s = formatdoc! {r#"
      10:00:00:00:00:aa 10.0.0.1 host1.net ; Comment
      10:00:00:00:00:bb 10.0.0.2 host1.net
      10:00:00:00:00:cc 10.0.0.3 host3.net

      {EPILOGUE}
    "#};
    check_err!(proc(&s), "✗ Host name host1.net on line 2 is duplicate of line 1");
}
