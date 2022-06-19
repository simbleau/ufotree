// Parser for metainfo.plist
use crate::{errors::ParseError, parsers::Parse};

struct MetaInfo {
    creator: String,
    format_version: u8,
    format_version_minor: u8,
}

impl Parse for MetaInfo {
    type Err = ParseError;

    fn parse(source: &String) -> Result<Self, Self::Err> {
        Ok(MetaInfo {
            creator: todo!(),
            format_version: todo!(),
            format_version_minor: todo!(),
        })
    }
}
