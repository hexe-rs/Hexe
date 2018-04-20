macro_rules! parse_error {
    ($val:expr, $err:expr) => {
        error!("Could not parse \"{}\": {}", $val, $err);
    };
}
