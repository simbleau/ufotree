pub trait Parse: Sized {
    type Err;
    fn parse(source: &String) -> Result<Self, Self::Err>;
}

pub mod ufo3;
