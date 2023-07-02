#[macro_export]
macro_rules! return_if_error {
    // macth like arm for macro
    ($a:expr,$b:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            let result = $a;
            match result {
                Ok(ok) => ok,
                Err(_) => return $b,
            }
        }
    };
}
