use std::{backtrace::Backtrace, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid path")]
    Io {
        #[from]
        source: io::Error,
        backtrace: Backtrace,
    },
}
