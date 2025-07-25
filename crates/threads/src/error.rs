//! Error & Result type for Day07 of Advent of Code 2024.
//!
//! ## Utility reference
//! For adding backtrace to errors:
//! `#![feature(error_generic_member_access)]`
//! `use std::backtrace;`

use std::io;

use derive_more::{Display, Error};
use tracing::{instrument, subscriber::SetGlobalDefaultError};

// use derive_more::{Display, Error, derive::From};
#[derive(Debug, Display, derive_more::From, Error)]
pub enum ErrKind {
       Clap {
              source: clap::Error,
       },
       EnvError {
              source: tracing_subscriber::filter::FromEnvError,
       },
       HiddenValError {
              source: utilities::HiddenValueError,
       },
       Io {
              source: io::Error,
       },
       ParseInt {
              source: std::num::ParseIntError,
       },
       TracingSubscriber {
              source: SetGlobalDefaultError,
       },
       #[from(ignore)] // use `make_dyn_error` instead; would conflict with auto-derives
       #[display("Uncategorized Error (dyn error object): {}", source)]
       OtherErrorDyn {
              source: Box<dyn std::error::Error + Send + Sync>,
       },
       #[display(r#"Uncategorized string err: "{}""#, source_string)]
       OtherErrorString {
              source_string: String,
       },
}
impl ErrKind {
       /// Convenience asscfunction for transforming an error into a compabtible *dyn error*.
       ///
       /// ```ignore
       /// use support::ErrKind;
       /// let clip = arboard::Clipboard::new().map_err(ErrKind::into_dyn_error)?;
       /// ```
       #[instrument(skip_all)]
       pub fn into_dyn_error<E>(error: E) -> Self
       where
              E: Into<Box<dyn std::error::Error + Send + Sync>>,
       {
              Self::OtherErrorDyn { source: error.into() }
       }
}

#[derive(Display, Error)]
#[display(
        "error: {:#}\n\n\nspantrace capture: {:?}\n\n\nspantrace: {:#}",
        source,
        spantrace.status(),
        spantrace,
)]
pub struct ErrWrapper {
       source:    ErrKind,
       spantrace: tracing_error::SpanTrace,
       // backtrace: backtrace::Backtrace,
}
// Using custom display as debug so we can get SpanTrace auto printed.
impl std::fmt::Debug for ErrWrapper {
       #[instrument(skip_all)]
       #[expect(unused_braces)]
       fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self) }
}
impl<E> From<E> for ErrWrapper
where
       E: Into<ErrKind>,
{
       #[instrument(skip_all)]
       fn from(error: E) -> Self {
              Self {
                     source:    error.into(),
                     spantrace: tracing_error::SpanTrace::capture(),
                     // backtrace: backtrace::Backtrace::capture(),
              }
       }
}
