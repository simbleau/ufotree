#![feature(backtrace)]
use std::path::Path;

mod errors;
mod models;

pub fn parse_ufo<P>(dir_path: P) -> Result<models::UfoTree, errors::ParseError>
where
    P: AsRef<Path>,
{
    todo!()
}

pub fn parse_ufoz<P>(zip_path: P) -> Result<models::UfoTree, errors::ParseError>
where
    P: AsRef<Path>,
{
    todo!()
}

#[cfg(test)]
#[test]
pub fn it_works() {
    assert_eq!(true, true);
}
