use super::error::Result;
use std::io::Write;

pub trait Encode {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize>;
}
