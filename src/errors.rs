use std::{backtrace::Backtrace, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("placeholder")]
    Placeholder,
}
