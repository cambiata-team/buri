#[macro_export]
macro_rules! return_if_error {
    ($a:expr,$b:expr) => {{
        let result = $a;
        match result {
            Ok(ok) => ok,
            Err(_) => return $b,
        }
    }};
}
