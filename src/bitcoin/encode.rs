use crate::bitcoin::Result;
use std::io::Write;

pub trait Encode {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize>;
}

impl Encode for u16 {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(2)
    }
}

impl Encode for u32 {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(4)
    }
}

impl Encode for u64 {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(8)
    }
}

impl Encode for i32 {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(4)
    }
}

impl Encode for i64 {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize> {
        buffer.write_all(&self.to_le_bytes())?;
        Ok(8)
    }
}

impl Encode for bool {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize> {
        buffer.write_all(&[*self as u8])?;
        Ok(1)
    }
}

impl Encode for () {
    fn encode(&self, _buffer: &mut impl Write) -> Result<usize> {
        Ok(0)
    }
}

impl Encode for std::net::IpAddr {
    fn encode(&self, buffer: &mut impl Write) -> Result<usize> {
        use std::net::IpAddr::*;
        match self {
            V4(ip) => buffer.write_all(&[[0; 4], [0; 4], [0; 4], ip.octets()].concat())?,
            V6(ip) => buffer.write_all(&ip.octets())?,
        }
        Ok(16)
    }
}
