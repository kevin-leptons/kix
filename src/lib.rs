//! A result type for testing that supports to eliminate using of
//! `Result::unwrap`. So a library may enforce flags such as `-D
//! clippy::unwrap_used` without hassle.
//!
//! Set environment `RUST_BACKTRACE=1` to enable backtrace. The first
//! backtrace's frame does not point to error's location, the third does. This
//! issue may be solved once
//! [Backtrace::frames](std::backtrace::Backtrace::frames) are stable.
//!
//! Do not use [Result] for anything other than testing. Because it depends on
//! [Error] that does not implement [std::error::Error]. This is not a choice.
//! There is no way to implemnt `std::error::Error` and `From<dyn
//! std::error::Error>` at the same time. If there is a need of using [Error] as
//! [std::error::Error], then [Error::as_std_error] may help.
//!
//! ```no_run
#![doc = include_str!("../examples/unit_test.rs")]
//!```

use std::backtrace::{Backtrace, BacktraceStatus};
use std::fmt::{Debug, Display};

pub type Result = std::result::Result<(), Error>;

pub struct Error {
    inner: Box<InnerError>,
}

struct InnerError {
    source: Box<dyn std::error::Error>,
    trace: Backtrace,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl<E> From<E> for Error
where
    E: std::error::Error + 'static,
{
    fn from(error: E) -> Self {
        let inner = InnerError {
            source: error.into(),
            trace: Backtrace::capture(),
        };
        Self {
            inner: Box::new(inner),
        }
    }
}

impl Error {
    pub fn as_std_error(&self) -> &(dyn std::error::Error + 'static) {
        &self.inner
    }
}

impl From<Error> for Box<dyn std::error::Error> {
    fn from(error: Error) -> Self {
        error.inner
    }
}

impl std::error::Error for InnerError {}

impl Display for InnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { source, trace } = self;
        write!(f, "{source}\nBacktrace: ")?;
        match self.trace.status() {
            BacktraceStatus::Captured => write!(f, "\n{trace}"),
            BacktraceStatus::Unsupported => write!(f, "Unsupported."),
            BacktraceStatus::Disabled => write!(
                f,
                "Disabled. Turn on by environment variable 'RUST_BACKTRACE=1'."
            ),
            _ => write!(f, "Unknown."),
        }
    }
}

impl Debug for InnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}
