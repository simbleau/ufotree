mod metainfo;

pub trait Parse: Sized {
    type Err;
    fn parse(source: &String) -> Result<Self, Self::Err>;
}
