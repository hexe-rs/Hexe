macro_rules! log_trace {
    ($($t:tt)*) => {
        #[cfg(feature = "log")]
        trace! { $($t)* }
    }
}
