/// Various utils used in all test files

#[macro_export]
macro_rules! check_err {
    ($val:expr, $msg:literal) => {{
        let res = &$val;
        assert!(res.is_err());
        assert_eq!($msg, res.as_ref().unwrap_err().to_string());
    }};
}
