use crate::bitcoin::Result;

pub trait Decode
where
    Self: Sized,
{
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])>;
}

impl Decode for () {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        Ok(((), bytes))
    }
}

impl Decode for bool {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        Ok((bytes[0] != 0, &bytes[1..]))
    }
}

impl Decode for u16 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        Ok((Self::from_le_bytes(bytes[..2].try_into()?), &bytes[2..]))
    }
}

impl Decode for u32 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        Ok((Self::from_le_bytes(bytes[..4].try_into()?), &bytes[4..]))
    }
}

impl Decode for u64 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        Ok((Self::from_le_bytes(bytes[..8].try_into()?), &bytes[8..]))
    }
}

impl Decode for i16 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        Ok((Self::from_le_bytes(bytes[..2].try_into()?), &bytes[2..]))
    }
}

impl Decode for i32 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        Ok((Self::from_le_bytes(bytes[..4].try_into()?), &bytes[4..]))
    }
}

impl Decode for i64 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        Ok((Self::from_le_bytes(bytes[..8].try_into()?), &bytes[8..]))
    }
}

impl Decode for std::net::IpAddr {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        let ip = TryInto::<[u8; 16]>::try_into(&bytes[..16])?.into();
        Ok((ip, &bytes[16..]))
    }
}
