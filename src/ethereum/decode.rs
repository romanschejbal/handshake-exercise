use super::error::Result;
use std::io::Read;

pub trait Decode
where
    Self: Sized,
{
    fn decode(bytes: &mut impl Read) -> Result<(Self, &[u8])>;
}
