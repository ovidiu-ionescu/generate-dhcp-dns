use ::generate_dhcp_dns::principal;
use indoc::indoc;

macro_rules! check_err {
    ($val:expr, $msg:literal) => {{
        let res = &$val;
        assert!(res.is_err());
        assert_eq!($msg, res.as_ref().unwrap_err().to_string());
    }};
}

#[test]
fn test_duplicate_ip() {
    let s = indoc! {"
      10:00:00:00:00:aa 10.0.0.1 host1.net ; Comment
      10:00:00:00:00:bb 10.0.0.1 host2.net
      10:00:00:00:00:cc 10.0.0.3 host3.net

      domain foo.net
      dns_file_name db.foo
      reverse_dns_file_name db.0.0.10
      dhcp_file_name reservations.conf
    "};
    check_err!(principal(s), "Ip address 10.0.0.1 on line 2 is duplicate of line 1");
}

#[test]
fn test_duplicate_mac() {
    let s = indoc! {"
      10:00:00:00:00:aa 10.0.0.1 host1.net ; Comment
      10:00:00:00:00:aa 10.0.0.2 host2.net
      10:00:00:00:00:cc 10.0.0.3 host3.net

      domain foo.net
      dns_file_name db.foo
      reverse_dns_file_name db.0.0.10
      dhcp_file_name reservations.conf
    "};
    check_err!(principal(s), "Mac address 10:00:00:00:00:aa on line 2 is duplicate of line 1");
}

#[test]
fn test_duplicate_hostname() {
    let s = indoc! {"
      10:00:00:00:00:aa 10.0.0.1 host1.net ; Comment
      10:00:00:00:00:bb 10.0.0.2 host1.net
      10:00:00:00:00:cc 10.0.0.3 host3.net

      domain foo.net
      dns_file_name db.foo
      reverse_dns_file_name db.0.0.10
      dhcp_file_name reservations.conf
    "};
    check_err!(principal(s), "Host name host1.net on line 2 is duplicate of line 1");
}
