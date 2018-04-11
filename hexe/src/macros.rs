#[cfg(not(feature = "log"))]
macro_rules! trace { ($($t:tt)*) => {} }

#[cfg(not(feature = "log"))]
macro_rules! debug { ($($t:tt)*) => {} }

#[cfg(not(feature = "log"))]
macro_rules! info { ($($t:tt)*) => {} }

#[cfg(not(feature = "log"))]
macro_rules! warn { ($($t:tt)*) => {} }

#[cfg(not(feature = "log"))]
macro_rules! error { ($($t:tt)*) => {} }
