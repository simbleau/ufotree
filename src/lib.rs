#![feature(backtrace)]
use std::path::Path;

mod errors;
mod models;
mod parsers;

pub fn parse_ufo<P>(dir_path: P) -> Result<models::UfoTree, errors::ParseError>
where
    P: AsRef<Path>,
{
    todo!()
    // General instructions:
    // Call loader to get version number (1, 2, 3)
    // Get parser from version number
    // Parse with UFO X parser
    // Return result
}

pub fn parse_ufoz<P>(zip_path: P) -> Result<models::UfoTree, errors::ParseError>
where
    P: AsRef<Path>,
{
    todo!()
    // General instructions:
    // Unzip folder
    // Call loader to get version number (1, 2, 3)
    // Get parser from version number
    // Parse with UFO X parser
    // Return result
}
