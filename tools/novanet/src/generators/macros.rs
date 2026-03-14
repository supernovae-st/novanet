//! Infallible write macros for String buffer generation.
//!
//! Writing to a `String` via `std::fmt::Write` cannot fail, so the
//! `.unwrap()` on `writeln!`/`write!` is pure noise.  These macros
//! hide the unwrap while keeping the call-site readable.

/// Write a formatted line to a String buffer (infallible).
///
/// Equivalent to `writeln!(buf, …).unwrap()` but shorter.
macro_rules! cy {
    ($dst:expr) => {
        writeln!($dst).unwrap()
    };
    ($dst:expr, $($arg:tt)*) => {
        writeln!($dst, $($arg)*).unwrap()
    };
}

/// Write formatted text to a String buffer **without** a trailing newline.
///
/// Equivalent to `write!(buf, …).unwrap()`.
macro_rules! cyw {
    ($dst:expr, $($arg:tt)*) => {
        write!($dst, $($arg)*).unwrap()
    };
}
